use reqwest::blocking::get;
use serde::Serialize;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::thread;
use std::time::Duration;
use tauri::Manager;
use tauri::{AppHandle, Runtime};

use crate::internal;

#[derive(Serialize, Debug)] // Debug for printing to console
pub enum FeedItem {
    Rss(RssEntry),
    Atom(AtomEntry),
}

#[derive(Clone, Debug, Serialize)] // Debug for printing to console
pub enum FeedType {
    RSS,
    ATOM,
}

#[derive(Serialize, Debug)] // Debug for printing to console
pub struct RssEntry {
    // Required fields
    pub title: String,
    pub hash: i64,
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

impl RssEntry {
    pub fn new(
        title: String,
        link: Option<String>,
        description: Option<String>,
        pub_date: Option<String>,
        author: Option<String>,
        category: Option<String>,
        comments: Option<String>,
        enclosure: Option<String>,
        guid: Option<String>,
    ) -> Self {
        let mut hasher = DefaultHasher::new();
        title.hash(&mut hasher);
        link.hash(&mut hasher);
        description.hash(&mut hasher);
        pub_date.hash(&mut hasher);
        author.hash(&mut hasher);
        category.hash(&mut hasher);
        comments.hash(&mut hasher);
        enclosure.hash(&mut hasher);
        guid.hash(&mut hasher);
        let hash = hasher.finish() as i64;

        RssEntry {
            title,
            link,
            description,
            pub_date,
            author,
            category,
            comments,
            enclosure,
            guid,
            hash,
        }
    }
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
    pub hash: i64,
}

impl AtomEntry {
    pub fn new(
        title: String,
        link: Option<String>,
        summary: Option<String>,
        id: Option<String>,
        updated: Option<String>,
        author: Option<String>,
        category: Option<String>,
        content: Option<String>,
        contributor: Option<String>,
        pub_date: Option<String>,
        rights: Option<String>,
    ) -> Self {
        let mut hasher = DefaultHasher::new();

        // Hash the title
        title.hash(&mut hasher);

        // Hash each optional field if it exists
        let fields_to_hash = [
            &link,
            &summary,
            &id,
            &updated,
            &author,
            &category,
            &content,
            &contributor,
            &pub_date,
            &rights,
        ];

        for field in fields_to_hash.iter() {
            if let Some(value) = field {
                value.hash(&mut hasher);
            }
        }

        let hash = hasher.finish() as i64;

        AtomEntry {
            title,
            link,
            summary,
            id,
            updated,
            author,
            category,
            content,
            contributor,
            pub_date,
            rights,
            hash,
        }
    }
}

#[derive(Clone, Debug, Serialize)] // Debug for printing to console
pub struct Feed {
    pub url: String,
    pub feed_type: FeedType,
    pub poll_interval: i32,
    pub alias: Option<String>,
}

impl Feed {
    pub fn new(
        feed_url: String,
        feed_alias: Option<String>,
        poll_interval: i32,
        feed_type: FeedType,
    ) -> Self {
        // changed to handle values in seconds
        Self {
            url: feed_url,
            alias: feed_alias,
            poll_interval,
            feed_type,
        }
    }
}

fn fetch_feed(
    url: &str,
    feed_type: &FeedType,
) -> Result<Vec<FeedItem>, Box<dyn std::error::Error>> {
    let new_feed_item = match feed_type {
        FeedType::RSS => fetch_rss(url).expect("Error fetching RSS feed item"),
        FeedType::ATOM => fetch_atom(url).expect("Error fetching Atom feed item"),
    };
    // insert newly fetched item into DB
    internal::sqlite_db::put_feed_items_db(new_feed_item)
        .expect("Error sending fetched feed to DB");

    Ok(vec![])
}

fn fetch_rss(url: &str) -> Result<Vec<FeedItem>, Box<dyn std::error::Error>> {
    println!("Attempting to Fetch Feed from URL: {url}");
    let response = get(url)?.text()?;
    // println!("{:?}", response);

    let channel = rss::Channel::read_from(response.as_bytes())?;

    let items: Vec<FeedItem> = channel
        .items()
        .iter()
        .filter_map(|item| {
            let mut title = item.title().map(|s| s.to_string());
            if title.is_none() {
                title = Some(channel.title().to_string());
            }
            Some(FeedItem::Rss(RssEntry::new(
                title.unwrap(),
                item.link().map(|s| s.to_string()),
                item.description().map(|s| s.to_string()),
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

            // Create AtomEntry instance using new constructor
            let atom_entry = AtomEntry::new(
                title.clone(),
                entry.links().first().map(|link| link.href().to_string()),
                entry.summary().map(|summary| summary.to_string()),
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

            Some(FeedItem::Atom(atom_entry))
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
    let items = internal::sqlite_db::get_feed_items_db(); // pull items from DB, not directly from fetch
    app.emit_all("new-rss-items", &items)?; // NO EMIT, that is done elsewhere, direct from DB

    Ok(())
}

// There should be another thread
// Bulk update (not little constant bombardments to FE)
// FE listener, Every 30 sec render everything in the DB that is new
