use reqwest::blocking::get;
use serde::Serialize;
use std::thread;
use std::time::Duration;
use tauri::Manager;
use tauri::{AppHandle, Runtime};

use crate::internal::dbo::entry::{AtomEntry, FeedEntryType, RssEntry};
use crate::internal::sqlite_db::{get_feed_items_db, put_feed_items_db, get_atom_feeds_db, get_rss_feeds_db};

#[derive(Clone, Debug, Serialize)] // Debug for printing to console
pub enum FeedType {
    RSS,
    ATOM,
}

#[derive(Clone, Debug, Serialize)] // Debug for printing to console
pub struct Feed {
    pub url: String,
    pub feed_type: FeedType,
    pub poll_interval: i32,
    pub alias: Option<String>,
    pub feed_id: Option<usize>,
}

impl Feed {
    pub fn new(
        feed_url: String,
        feed_alias: String,
        poll_interval: i32,
        feed_type: FeedType,
        feed_id: usize,
    ) -> Self {
        // changed to handle values in seconds
        Self {
            url: feed_url,
            alias: Some(feed_alias),
            poll_interval,
            feed_type,
            feed_id: Some(feed_id),
        }
    }
}

fn fetch_feed(
    url: &str,
    feed_type: &FeedType,
) -> Result<Vec<FeedEntryType>, Box<dyn std::error::Error>> {
    // Get feed_id of parent feed for pk/fk matching
    match feed_type {
        FeedType::RSS => {
            let feed_vec = get_rss_feeds_db();
            match feed_vec {
                Ok(feed_vec) => {
                    // println!("Yea");
                    // let feed = feed_vec.into_iter().nth(0).unwrap(); //feed_vec.first();
                    let feed = feed_vec.first().unwrap();
                    // println!("{:?}", feed.feed_id);
                    let new_feed_item = fetch_rss(url, feed.feed_id).expect("Error fetching RSS feed item");
                    put_feed_items_db(new_feed_item).expect("Error sending fetched feed to DB");
                }
                Err(_) => println!("Shit")
            } 
        }
        FeedType::ATOM => {
            let feed_vec = get_atom_feeds_db();
            match feed_vec {
                Ok(feed_vec) => {
                    let feed = feed_vec.first().unwrap();
                    let new_feed_item = fetch_atom(url, feed.feed_id).expect("Error fetching Atom feed item");
                    put_feed_items_db(new_feed_item).expect("Error sending fetched feed to DB");
                }
                Err(_) => println!("Shit")
            }
        }
    };

    // let new_feed_item = match feed_type {
    //     FeedType::RSS => fetch_rss(url).expect("Error fetching RSS feed item"),
    //     FeedType::ATOM => fetch_atom(url).expect("Error fetching Atom feed item"),
    // };
    
    // insert newly fetched item into DB
    // put_feed_items_db(new_feed_item).expect("Error sending fetched feed to DB");

    Ok(vec![])
}

fn fetch_rss(url: &str, feed_id: Option<usize>) -> Result<Vec<FeedEntryType>, Box<dyn std::error::Error>> {
    let response = get(url)?.text()?;
    // println!("{:?}", response);

    let channel = rss::Channel::read_from(response.as_bytes())?;

    let items: Vec<FeedEntryType> = channel
        .items()
        .iter()
        .filter_map(|item| {
            let mut title = item.title().map(|s| s.to_string());
            if title.is_none() {
                title = Some(channel.title().to_string());
            }
            Some(FeedEntryType::RSS(RssEntry::new(
                title.unwrap(),
                item.link().map(|s| s.to_string()),
                item.description().map(|s| s.to_string()),
                feed_id,
                item.pub_date().map(|s| s.to_string()),
                item.author().map(|s| s.to_string()),
                item.categories().first().map(|c| c.name().to_string()),
                item.comments().map(|s| s.to_string()),
                item.enclosure().map(|e| e.url().to_string()),
                item.guid().map(|g| g.value().to_string()),
            )))
        })
        .collect();

    Ok(items)
}

fn fetch_atom(url: &str, feed_id: Option<usize>) -> Result<Vec<FeedEntryType>, Box<dyn std::error::Error>> {
    let response = get(url)?.text()?;
    let feed = atom_syndication::Feed::read_from(response.as_bytes())?;

    let items: Vec<FeedEntryType> = feed
        .entries()
        .iter()
        .filter_map(|entry| {
            let title = entry.title().to_string();
            if title.is_empty() {
                return None;
            }

            // Create AtomEntry instance using new constructor
            let atom_entry = AtomEntry::new(
                title.clone(),
                entry.links().first().map(|link| link.href().to_string()),
                entry.summary().map(|summary| summary.to_string()),
                feed_id,
                Some(entry.id().to_string()),
                Some(entry.updated().to_string()),
                entry
                    .authors()
                    .first()
                    .map(|person| person.name().to_string()),
                entry
                    .categories()
                    .first()
                    .map(|category| category.term().to_string()),
                entry
                    .content()
                    .map(|content| content.value().unwrap_or_default().to_string()),
                entry
                    .contributors()
                    .first()
                    .map(|person| person.name().to_string()),
                entry.published.map(|pub_date| pub_date.to_string()),
                entry.rights().map(|rights| rights.to_string()),
            );

            Some(FeedEntryType::ATOM(atom_entry))
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
            thread::sleep(Duration::from_secs(feed_clone.poll_interval as u64));
        });
    }
}

fn fetch_and_emit_feed<R: Runtime>(
    // This just becomes fetch, no emit
    app: &AppHandle<R>,
    feed: &Feed,
) -> Result<(), Box<dyn std::error::Error>> {
    // let items = fetch_feed(&feed.url, &feed.feed_type)?; // To be deleted
    let _x = fetch_feed(&feed.url, &feed.feed_type); // THIS IS ERRORING on RSS item fetch, will send fetched items to DB
    let items = get_feed_items_db(); // pull items from DB, not directly from fetch
    app.emit_all("new-rss-items", &items)?; // NO EMIT, that is done elsewhere, direct from DB

    Ok(())
}

// There should be another thread
// Bulk update (not little constant bombardments to FE)
// FE listener, Every 30 sec render everything in the DB that is new
