use reqwest::blocking::get;
use serde::Serialize;
use std::thread;
use std::time::Duration;
use tauri::Manager;
use tauri::{AppHandle, Runtime};

#[derive(Serialize, Debug)]
pub struct FeedItem {
    title: String,
    link: String,
    description: String, // TODO: fill in the rest of these making some optional while others should be required
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

fn fetch_feed(
    url: &str,
    feed_type: &FeedType,
) -> Result<Vec<FeedItem>, Box<dyn std::error::Error>> {
    match feed_type {
        FeedType::RSS => fetch_rss(url),
        FeedType::ATOM => fetch_atom(url),
    }
}

fn fetch_rss(url: &str) -> Result<Vec<FeedItem>, Box<dyn std::error::Error>> {
    let response = get(url)?.text()?;
    let channel = rss::Channel::read_from(response.as_bytes())?;

    let items: Vec<FeedItem> = channel
        .items()
        .iter()
        .filter_map(|item| {
            if item.title().is_none() {
                None
            } else {
                Some(FeedItem {
                    title: item.title().unwrap_or_default().to_string(),
                    link: item.link().unwrap_or_default().to_string(),
                    description: item.description().unwrap_or_default().to_string(),
                })
            }
        })
        .collect();

    Ok(items)
}

fn fetch_atom(url: &str) -> Result<Vec<FeedItem>, Box<dyn std::error::Error>> {
    let response = get(url)?.text()?;
    let feed = atom_syndication::Feed::read_from(response.as_bytes())?;

    let items: Vec<FeedItem> = feed
        .entries()
        .iter()
        .filter_map(|entry| {
            let title = entry.title().to_string();
            if title.is_empty() {
                return None;
            }

            Some(FeedItem {
                title,
                link: entry
                    .links()
                    .first()
                    .map(|link| link.href().to_string())
                    .unwrap_or_default(),
                description: entry
                    .summary()
                    .map(|summary| summary.to_string())
                    .unwrap_or_default(),
            })
        })
        .collect();
    Ok(items)
}

pub fn start_feed_fetcher<R: Runtime>(app: AppHandle<R>, feeds: Vec<Feed>) {
    for feed in feeds {
        let app_handle = app.clone();
        let feed_clone = feed.clone();

        // Fetch feed immediately upon initialization
        if let Err(e) = fetch_and_emit_feed(&app_handle, &feed_clone) {
            eprintln!("Error fetching feed immediately: {}", e);
        }

        thread::spawn(move || loop {
            if let Err(e) = fetch_and_emit_feed(&app_handle, &feed_clone) {
                app_handle
                    .emit_all(
                        "feed-error",
                        &format!("Error fetching feed {}: {}", feed_clone.url, e),
                    )
                    .unwrap();
                eprintln!("Error fetching feed {}: {}", feed_clone.url, e);
            }
            thread::sleep(feed_clone.poll_interval);
        });
    }
}

fn fetch_and_emit_feed<R: Runtime>(
    app: &AppHandle<R>,
    feed: &Feed,
) -> Result<(), Box<dyn std::error::Error>> {
    let items = fetch_feed(&feed.url, &feed.feed_type)?;
    app.emit_all("new-rss-items", &items)?;
    Ok(())
}
