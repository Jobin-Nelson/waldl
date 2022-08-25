// use futures::{stream, StreamExt};
use futures::future::join_all;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::task::JoinHandle;

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

    let wallpaper_path: PathBuf = ["/home/jobin/Pictures/Wallpapers", &today].iter().collect();

    fs::create_dir_all(wallpaper_path.as_path())
        .await
        .expect("Could not create directory");

    // let client = reqwest::Client::new();
    // // let buffer_number = wallpaper_links.len();
    //
    // let responses = stream::iter(wallpaper_links).map(|url| {
    //     let client = client.clone();
    //     tokio::spawn(async move {
    //         let response = client.get(url).send().await?;
    //         response.bytes().await
    //     })
    // }).buffer_unordered(8);
    //
    // responses.enumerate().for_each(|(i, b)| async move {
    //     match b {
    //         Ok(Ok(b)) => {
    //             let file_path = "/home/jobin/Pictures/Wallpapers/aug_22/wallhaven_".to_owned() + &i.to_string();
    //             let file_name = Path::new(&file_path).file_name().unwrap();
    //             if let Ok(_) = fs::write(&file_path, b).await {
    //                 println!("Downloading file {}", file_name.to_str().unwrap());
    //             } else {
    //                 println!("Could not download skipping image");
    //             };
    //         },
    //         Ok(Err(e)) => eprintln!("Couldn't download image {}", e),
    //         Err(e) => eprintln!("Couldn't join tokio::JoinError {}", e),
    //     }
    // }).await;

    let mut tasks: Vec<JoinHandle<()>> = Vec::new();

    for link in wallpaper_links {
        let wallpaper_path = wallpaper_path.clone();

        tasks.push(tokio::spawn(async move {
            match reqwest::get(&link).await {
                Ok(response) => {
                    let file_name = Path::new(&link).file_name().unwrap();
                    let file_path = wallpaper_path.join(file_name);
                    if let Ok(_) =
                        fs::write(file_path.as_path(), response.bytes().await.unwrap()).await
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
