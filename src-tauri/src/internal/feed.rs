use reqwest::blocking::get;
use serde::Serialize;
use std::thread;
use std::time::Duration;
use tauri::Manager;
use tauri::{AppHandle, Runtime};

#[derive(Serialize)]
pub enum FeedItem{
    Rss(RssEntry),
    Atom(AtomEntry)
}

#[derive(Clone)]
pub enum FeedType {
    RSS,
    ATOM,
}

#[derive(Serialize)]
pub struct RssEntry {
    // Required fields
    pub title: String,
    pub link: Option<String>,
    pub description: Option<String>,
    // Optional fields
    pub pub_date: Option<String>,
    pub author: Option<String>,
    pub category: Option<String>,
    pub comments: Option<String>,
    pub enclosure: Option<String>,
    pub guid: Option<String>,
}

// Struct for Atom item
#[derive(Serialize)]
pub struct AtomEntry {
    // Required fields
    pub title: String,
    pub link: Option<String>,
    pub summary: Option<String>,
    // Optional fields
    pub id: Option<String>,
    pub updated: Option<String>,
    pub author: Option<String>,
    pub category: Option<String>,
    pub content: Option<String>,
    pub contributor: Option<String>,
    pub published: Option<String>,
    pub rights: Option<String>,
}

#[derive(Clone)]
pub struct Feed {
    pub url: String,
    pub feed_type: FeedType,
    pub poll_interval: Duration,
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
            let title = item.title().map(|s| s.to_string());
            if title.is_none() {
                return None;
            }

            Some(FeedItem::Rss(RssEntry {
                title: title.unwrap(),
                link: item.link().map(|s| s.to_string()),
                description: item.description().map(|s| s.to_string()),
                pub_date: item.pub_date().map(|s| s.to_string()),
                author: item.author().map(|s| s.to_string()),
                category: item.categories().first().map(|c| c.name().to_string()),
                comments: item.comments().map(|s| s.to_string()),
                enclosure: item.enclosure().map(|e| e.url().to_string()),
                guid: item.guid().map(|g| g.value().to_string()),
            }))
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

            Some(FeedItem::Atom(AtomEntry {
                title,
                link: entry.links().first().map(|link| link.href().to_string()),
                summary: entry.summary().map(|summary| summary.to_string()),
                id: Some(entry.id().to_string()),
                updated: Some(entry.updated().to_string()),
                author: entry.authors().first().map(|person| person.name().to_string()),
                category: entry.categories().first().map(|category| category.term().to_string()),
                content: entry.content().map(|content| content.value().unwrap_or_default().to_string()),
                contributor: entry.contributors().first().map(|person| person.name().to_string()),
                published: entry.published.map(|pub_date| pub_date.to_string()),
                rights: entry.rights().map(|rights| rights.to_string()),
            }))
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
