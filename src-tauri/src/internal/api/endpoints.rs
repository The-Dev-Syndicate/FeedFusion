use tauri::command;
use crate::internal::{core::core_functions, dbo::feed::Feed};

#[command] // Tauri decorator
pub fn greet(name: &str) -> String { core_functions::greet(name) }

#[command] // Tauri decorator
pub fn load_feeds() -> Vec<Feed> { core_functions::load_feeds() }

#[command] // Tauri decorator
pub fn add_feed(feed_url: String, feed_alias: String, poll_timer: i32) -> Result<(), String> { core_functions::add_feed(feed_url, feed_alias, poll_timer) } // Changed to i32 to handle intervals in seconds
