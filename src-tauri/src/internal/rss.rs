use serde::Serialize;
use std::time::Duration;
use tauri::Manager;
use tauri::{AppHandle, Runtime};
use reqwest::blocking::get;
use std::thread;

#[derive(Serialize)]
pub struct FeedItem{
    title: String,
    link: String,
    description: String
    // TODO: fill in the rest of these making some optional while others should be required 
    // (see RSS Specs : https://www.rssboard.org/rss-specification#requiredChannelElements)
    // (see ATOM Specs: https://validator.w3.org/feed/docs/atom.html)
}

#[derive(Clone)]
pub struct Feed {
    pub url: String,
    pub feed_type: FeedType,
    pub poll_interval: Duration,
}

#[derive(Clone)]
pub enum FeedType {
    RSS,
    ATOM,
}

fn fetch_feed(url: &str, feed_type: &FeedType) -> Result<Vec<FeedItem>, Box<dyn std::error::Error>> {
    match feed_type {
        FeedType::RSS => fetch_rss(url),
        FeedType::ATOM => fetch_atom(url),
    }
}

fn fetch_rss(url: &str) -> Result<Vec<FeedItem>, Box<dyn std::error::Error>> {
    let response = get(url)?.text()?;
    let channel = rss::Channel::read_from(response.as_bytes())?;

    let items: Vec<FeedItem> = channel.items()
        .iter()
        .map(|item| FeedItem {
            title: item.title().unwrap_or_default().to_string(),
            link: item.link().unwrap_or_default().to_string(),
            description: item.description().unwrap_or_default().to_string(),
        })
        .collect();

    Ok(items)
}

fn fetch_atom(url: &str) -> Result<Vec<FeedItem>, Box<dyn std::error::Error>> {
    // TODO: Implement ATOM feed fetching logic here
    Ok(Vec::new())
}

pub fn start_feed_fetcher<R: Runtime>(app: AppHandle<R>, feeds: Vec<Feed>) {
    fetch_and_emit_feeds(&app, &feeds); // Do this only once then moves to normal interval polling

    thread::spawn(move || {
        loop {
            for feed in &feeds {
                match fetch_feed(&feed.url, &feed.feed_type) {
                    Ok(items) => {
                        app.emit_all("new-rss-items", &items).unwrap();
                    }
                    Err(e) => eprintln!("Error fetching feed {}: {}", feed.url, e),
                }
            }
            thread::sleep(Duration::from_secs(60)); // Fetch every 60 seconds
            println!("Going to sleep");
        }
    });
}

// This runs right when the app starts to ensure we get data on start 
fn fetch_and_emit_feeds<R: Runtime>(app: &AppHandle<R>, feeds: &[Feed]) {
    for feed in feeds {
        match fetch_feed(&feed.url, &feed.feed_type) {
            Ok(items) => {
                app.emit_all("new-rss-items", &items).unwrap();
            }
            Err(e) => eprintln!("Error fetching feed {}: {}", feed.url, e),
        }
    }
}
