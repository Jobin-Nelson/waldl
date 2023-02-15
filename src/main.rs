use futures::future::join_all;
use futures::stream::FuturesUnordered;
use std::path::{Path, PathBuf};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = &std::env::args().collect::<Vec<String>>()[1..].join(" ");

    let wallpaper_links = get_wallpaper_links(args).await?;

    download_wallpaper(wallpaper_links).await;

    Ok(())
}

async fn get_wallpaper_links(args: &String) -> Result<Vec<String>, reqwest::Error> {
    let search_url = if args == "" {
        String::from("https://wallhaven.cc/api/v1/search")
    } else {
        reqwest::Url::parse_with_params("https://wallhaven.cc/api/v1/search", &[("q", args)])
            .expect("Could not parse link")
            .to_string()
    };

    let response: serde_json::Value = reqwest::get(&search_url).await?.json().await?;

    let mut wallpaper_links: Vec<String> = Vec::new();

    for wallpaper in response["data"].as_array().unwrap() {
        wallpaper_links.push(wallpaper["path"].as_str().unwrap().to_owned());
    }

    Ok(wallpaper_links)
}

async fn download_wallpaper(wallpaper_links: Vec<String>) -> () {
    let today = chrono::Utc::now()
        .format("%b_%d")
        .to_string()
        .to_ascii_lowercase();

    let wallpaper_path: PathBuf = ["/home/jobin/Pictures/wallpapers", &today].iter().collect();

    std::fs::create_dir_all(wallpaper_path.as_path())
        .expect("Could not create directory");

    let tasks = FuturesUnordered::new();

    for link in wallpaper_links {
        let wallpaper_path = wallpaper_path.clone();

        tasks.push(tokio::spawn(async move {
            match reqwest::get(&link).await {
                Ok(response) => {
                    let file_name = Path::new(&link).file_name().unwrap();
                    let file_path = wallpaper_path.join(file_name);
                    if let Ok(_) =
                        std::fs::write(file_path.as_path(), response.bytes().await.unwrap())
                    {
                        println!("Downloading file {}", link);
                    } else {
                        println!("Could not write image {} to file", link);
                    }
                }
                Err(_) => {
                    println!("Could not download image from {}", link);
                }
            }
        }))
    }

    println!("Started {} tasks. Waiting...", tasks.len());
    join_all(tasks).await;
}
