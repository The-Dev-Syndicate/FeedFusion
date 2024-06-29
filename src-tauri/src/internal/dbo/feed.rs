use reqwest::blocking::get;
use serde::Serialize;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use tauri::Manager;
use tauri::{AppHandle, Runtime};

use crate::internal;
use crate::internal::sqlite_db::{self, put_atom_entry_db, put_rss_entry_db};

#[derive(Serialize, Debug)] // Debug for printing to console
pub enum FeedItem{
    Rss(RssEntry),
    Atom(AtomEntry),
}

#[derive(Clone, Debug)] // Debug for printing to console
pub enum FeedType {
    RSS,
    ATOM,
}

#[derive(Serialize, Debug)] // Debug for printing to console
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
#[derive(Serialize, Debug)] // Debug for printing to console
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
    pub pub_date: Option<String>,
    pub rights: Option<String>,
}

#[derive(Clone, Debug)] // Debug for printing to console
pub struct Feed {
    pub url: String,
    pub feed_type: FeedType,
    pub poll_interval: Duration,
}

fn fetch_feed(url: &str, feed_type: &FeedType) {
    let new_feed_item = match feed_type {
        FeedType::RSS => fetch_rss(url).expect("Error fetching RSS feed item"),
        FeedType::ATOM => fetch_atom(url).expect("Error fetching Atom feed item")
    };

    // insert newly fetched item into DB
    internal::sqlite_db::put_feed_items_db(new_feed_item).expect("Error sending fetched feed to DB");
}

fn fetch_rss(url: &str) -> Result<Vec<FeedItem>, Box<dyn std::error::Error>> {
    println!("Attempting to Fetch Feed from URL: {url}");
    let response = get(url)?.text()?; // for DB, go to DB not url
    println!("Response {response}");

    let channel = rss::Channel::read_from(response.as_bytes())?;

    let items: Vec<FeedItem> = channel
        .items()
        .iter()
        .filter_map(|item| {
            let mut title = item.title().map(|s| s.to_string());
            if title.is_none() {
                title = Some(channel.title().to_string());
            }
            Some(FeedItem::Rss(RssEntry {
                // title: title.map(|s| s.to_string()),//unwrap(),
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

    let mut items: Vec<FeedItem> = feed
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
                author: entry
                    .authors()
                    .first()
                    .map(|person| person.name().to_string()),
                category: entry
                    .categories()
                    .first()
                    .map(|category| category.term().to_string()),
                content: entry
                    .content()
                    .map(|content| content.value().unwrap_or_default().to_string()),
                contributor: entry
                    .contributors()
                    .first()
                    .map(|person| person.name().to_string()),
                pub_date: entry.published.map(|pub_date| pub_date.to_string()),
                rights: entry.rights().map(|rights| rights.to_string()),
            }))
        })
        .collect();
    
    //###################################################################//
    // let items = sqlite_db::get_feed_items_db();
    // items.append(&mut items_db); // put em together
    //###################################################################//
    
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

fn fetch_and_emit_feed<R: Runtime>( // This just becomes fetch, no emit
    app: &AppHandle<R>,
    feed: &Feed,
) -> Result<(), Box<dyn std::error::Error>> {
    // let items = fetch_feed(&feed.url, &feed.feed_type)?;
    fetch_feed(&feed.url, &feed.feed_type);
    // app.emit_all("new-rss-items", &items)?; // NO EMIT, that is done elsewhere, direct from DB
    
    Ok(())
}

// There should be another thread
// Bulk update (not little constant bombardments to FE)
// FE listener, Every 30 sec render everything in the DB that is new
