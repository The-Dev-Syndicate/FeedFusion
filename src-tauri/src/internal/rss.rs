use serde::Serialize;
use std::time::Duration;
use tauri::Manager;
use tauri::{AppHandle, Runtime};
use reqwest::blocking::get;
use std::thread;

#[derive(Serialize)]
pub struct RssItem{
    title: String,
    link: String,
    description: String
    // TODO: fill in the rest of these making some optional while others should be required (see: https://www.rssboard.org/rss-specification#requiredChannelElements)
}


fn fetch_rss_feed() -> Result<Vec<RssItem>, Box<dyn std::error::Error>> {
    let url = ""; // TODO: Add your favorite RSS Feed this is just for iterating over tests
    let response = get(url)?.text()?;
    let channel = rss::Channel::read_from(response.as_bytes())?;

    let items: Vec<RssItem> = channel.items()
        .iter()
        .map(|item| RssItem {
            title: item.title().unwrap_or_default().to_string(),
            link: item.link().unwrap_or_default().to_string(),
            description: item.description().unwrap_or_default().to_string(),
        })
        .collect();

    Ok(items)
}

pub fn start_rss_fetcher<R: Runtime>(app: AppHandle<R>) {
    thread::spawn(move || {
        loop {
            match fetch_rss_feed() {
                Ok(items) => {
                    app.emit_all("new-rss-items", &items).unwrap();
                }
                Err(e) => eprintln!("Error fetching RSS feed: {}", e),
            }
            thread::sleep(Duration::from_secs(60)); // Fetch every 60 seconds
            println!("Going to sleep");
        }
    });
}