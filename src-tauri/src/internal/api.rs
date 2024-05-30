// File to house backend logic
use std::fs::{File, OpenOptions};
use tauri::command;

use crate::internal::feed_config::{Feed, Feeds, FEED_CONFIGURATION};


#[command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[command]
pub fn load_feeds() -> Vec<Feed> {
    let feeds_mutex = FEED_CONFIGURATION.lock().unwrap();
    let feeds = feeds_mutex.feeds.iter().cloned().collect();
    // No need to drop the mutex as it will be automatically released when it goes out of scope
    feeds
}

#[command]
pub fn add_feed_url(feed_url: String, feed_alias: String, poll_timer: u8) {
    let file_path = "../feeds.yaml"; // TODO: this needs to be put probably somewhere else but for now it works for testing
    let new_feed = Feed::new(
        feed_url,
        String::from("null_for_now"),
        feed_alias,
        poll_timer,
    );
    let mut feeds_mutex = FEED_CONFIGURATION.lock().unwrap();
    feeds_mutex.add_feed(new_feed.clone());
    let feeds: Feeds = feeds_mutex.clone(); // Clone the Feeds struct
    drop(feeds_mutex); // Release the lock as we don't need it anymore

    let mut file: File = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)
        .unwrap();

    // FIXME: IDK why but the yaml written out is not super human readable but serde gets it so tomorrows issue
    serde_yaml::to_writer(&mut file, &feeds).unwrap_or_else(|err| {
        eprintln!("Failed to write feeds to file: {}", err);
    });
}
