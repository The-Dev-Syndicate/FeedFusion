use log::debug;
use url::Url;

use crate::internal;
use crate::internal::dbo::feed::Feed;

#[derive(Debug)]
pub enum FeedError {
    InvalidUrl,
}

pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub fn load_feeds() -> Vec<Feed> {
    let feeds_db = internal::sqlite_db::get_feeds_db().expect("Issue pulling Feeds from DB for FE");

    return feeds_db;
}

pub fn add_feed(feed_url: String, feed_alias: String, poll_timer: i32) -> Result<(), String> {
    match validate_and_correct_url(&feed_url) {
        Ok(_) => {
            debug!(
                "Eventually we will build with {} - {}",
                feed_alias, poll_timer
            );

            // TODO match -> RSS function, Atom function
            internal::sqlite_db::put_rss_feed_db(feed_url, poll_timer, feed_alias)
                .expect("Error adding new feed");

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
