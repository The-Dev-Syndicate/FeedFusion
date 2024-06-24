use serde::{Deserialize, Serialize};
use std::collections::LinkedList;
use std::sync::Mutex;

// This ensures the config is a singleton in the app and not recreated 100x
lazy_static::lazy_static! {
    pub static ref FEED_CONFIGURATION: Mutex<Feeds> = Mutex::new(load_feeds().unwrap_or_else(|err| {
        eprintln!("Failed to load feeds from file: {}", err);
        Feeds::default()
    }));
}

fn load_feeds() -> Result<Feeds, Box<dyn std::error::Error>> {
    let file_path = "../feeds.yaml";
    let file = std::fs::File::open(file_path)?;
    let feeds: Feeds = serde_yaml::from_reader(file)?;
    let mut c = 0;
    for f in &feeds.feeds{
        c = c + 1;
        println!( "{}, {}", c, f.url);
    }

    Ok(feeds)
}

#[derive(Deserialize, Serialize, Clone, Debug)] // Debug for printing to console
pub struct Feed {
    pub category: String,
    pub url: String,
    pub poll_timer: i32, // changed to handle values in seconds
    pub alias: Option<String>,
}

impl Feed {
    pub fn new(feed_url: String, category: String, feed_alias: Option<String>, poll_timer: i32) -> Self { // changed to handle values in seconds
        Self {
            category: category,
            url: feed_url,
            alias: feed_alias,
            poll_timer,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Feeds {
    pub feeds: LinkedList<Feed>,
}

impl Feeds {
    pub fn add_feed(&mut self, new_feed: Feed) {
        self.feeds.push_back(new_feed);
    }
}

impl Default for Feeds {
    fn default() -> Self {
        Self {
            feeds: LinkedList::new(),
        }
    }
}
