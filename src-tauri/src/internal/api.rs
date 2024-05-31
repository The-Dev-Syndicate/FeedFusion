// File to house backend logic
use tauri::command;

use crate::internal::feed_config::{Feed, FEED_CONFIGURATION};


#[command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[command]
pub fn load_feeds() -> Vec<Feed> {
    // TODO: This will come from in memory DB eventually
    let f1: Feed = Feed::new(
        "https://feed2.is.fake".to_string(),
        "fake".to_string(),
        Some("Feed 1".to_string()),
        5
    );
    let f2: Feed = Feed::new(
        "https://feed1.is.fake".to_string(),
        "fake".to_string(),
        Some("Feed 2".to_string()),
        5
    );
    let f3: Feed = Feed::new(
        "https://feed3.is.fake".to_string(),
        "fake".to_string(),
        None,
        5
    );

    let mut feeds_mutex = FEED_CONFIGURATION.lock().unwrap();
    feeds_mutex.add_feed(f1);
    feeds_mutex.add_feed(f2);
    feeds_mutex.add_feed(f3);
    let feeds = feeds_mutex.feeds.iter().cloned().collect();
    // No need to drop the mutex as it will be automatically released when it goes out of scope
    feeds
}

// #[command]
// pub fn add_feed(feed_url: String, feed_alias: String, poll_timer: u8) {
//     // TODO: this will become an in memory DB
// }
