use std::path::PathBuf;
use tokio::io::copy;
use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    let args = std::env::args().collect::<Vec<String>>()[1..].join(" ");

    let Ok(wallpaper_links) = get_wallpaper_links(&args).await else {
        eprintln!("Error occured getting wallpaper links from wallhaven");
        return;
    };
    download_wallpaper(wallpaper_links).await;
}

async fn get_wallpaper_links(args: &str) -> Result<Vec<String>, reqwest::Error> {
    let search_url = if args.is_empty() {
        String::from("https://wallhaven.cc/api/v1/search")
    } else {
        reqwest::Url::parse_with_params("https://wallhaven.cc/api/v1/search", &[("q", args)])
            .expect("Could not parse link")
            .to_string()
    };

    let response: serde_json::Value = reqwest::get(&search_url).await?.json().await?;
    Ok(response["data"]
        .as_array()
        .unwrap()
        .iter()
        .map(|w| w["path"].as_str().unwrap().to_owned())
        .collect())
}

async fn download_wallpaper(wallpaper_links: Vec<String>) {
    let today = chrono::Utc::now()
        .format("%F")
        .to_string()
        .to_ascii_lowercase();

    let wallpaper_path: PathBuf = ["/home/jobin/Pictures/wallpapers", &today].iter().collect();

    tokio::fs::create_dir_all(wallpaper_path.as_path())
        .await
        .expect("Could not create directory");

    let mut set = JoinSet::new();
    let client = reqwest::Client::new();

    for link in wallpaper_links {
        let client = client.clone();
        let wallpaper_path = wallpaper_path.clone();

        set.spawn(async move {
            let Ok(response) = client.get(&link).send().await else {
                eprintln!("Could not download image from {}", link);
                return;
            };
            let file_name = response
                .url()
                .path_segments()
                .and_then(|segment| segment.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .unwrap_or("tmp.bin");
            let file_path = wallpaper_path.join(file_name);
            let Ok(mut fname) = tokio::fs::File::create(&file_path).await else {
                eprintln!("Could not create file {:?}", file_path.file_name());
                return;
            };
            let mut cursor = std::io::Cursor::new(response.bytes().await.unwrap());
            if copy(&mut cursor, &mut fname).await.is_err() {
                eprintln!("Could not write image to file {:?}", file_path.file_name());
                return;
            }
            println!("Downloaded image to {}", file_path.to_string_lossy());
        
        });
    }

    println!("Started {} tasks. Waiting...", set.len());
    while let Some(res) = set.join_next().await {
        res.unwrap();
    }
}
