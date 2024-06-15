// These functions are the supporting core of all API's
use url::Url;

use crate::internal::dbo::article::Article;
use crate::internal::feed_config::{Feed, FEED_CONFIGURATION};

#[derive(Debug)]
pub enum FeedError {
    InvalidUrl,
}


pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub fn get_articles() -> Vec<Article> {
    // TODO: This function can actually be removed in favor of feed.rs push flow
    // Return two hardcoded fake articles for now
    vec![
        Article::new(
            "First Article",
            "This is the description of the first article.",
            "John Doe",
            "2024-05-30T12:00:00",
        ),
        Article::new(
            "Second Article",
            "This is the description of the second article.",
            "Jane Smith",
            "2024-05-31T09:30:00",
        ),
    ]
}

pub fn load_feeds() -> Vec<Feed> {
    // TODO: This will come from in memory DB eventually
    let f1: Feed = Feed::new(
        "https://feed2.is.fake".to_string(),
        "fake".to_string(),
        Some("Feed 1".to_string()),
        5,
    );
    let f2: Feed = Feed::new(
        "https://feed1.is.fake".to_string(),
        "fake".to_string(),
        Some("Feed 2".to_string()),
        5,
    );
    let f3: Feed = Feed::new(
        "https://feed3.is.fake".to_string(),
        "fake".to_string(),
        None,
        5,
    );

    let mut feeds_mutex = FEED_CONFIGURATION.lock().unwrap();
    feeds_mutex.add_feed(f1);
    feeds_mutex.add_feed(f2);
    feeds_mutex.add_feed(f3);
    let feeds = feeds_mutex.feeds.iter().cloned().collect();
    // No need to drop the mutex as it will be automatically released when it goes out of scope
    feeds
}

pub fn add_feed(feed_url: String, feed_alias: String, poll_timer: u8) -> Result<(), String> {
    match validate_and_correct_url(&feed_url) {
        Ok(_) => {
            println!("Eventually we will build with {} - {}", feed_alias, poll_timer); // TODO: 
            // let new_feed = Feed {
            //     category: "null_for_now".to_string(),
            //     url: valid_url,
            //     alias: Some(feed_alias),
            //     poll_timer,
            // };
            Ok(())
        }
        Err(FeedError::InvalidUrl) => Err("Invalid URL".to_string()),
    }
}

// -------------------------------------- HELPERS --------------------------------------  \\
fn validate_and_correct_url(feed_url: &str) -> Result<String, FeedError> {
    if !feed_url.starts_with("http://") && !feed_url.starts_with("https://") {
        let corrected_url = format!("https://{}", feed_url);
        return validate_url(&corrected_url);
    }

    validate_url(feed_url)
}

fn validate_url(feed_url: &str) -> Result<String, FeedError> {
    match Url::parse(feed_url) {
        Ok(url) => Ok(url.to_string()),
        Err(_) => Err(FeedError::InvalidUrl),
    }
}