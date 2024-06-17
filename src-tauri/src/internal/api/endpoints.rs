use tauri::command;
use crate::internal::{dbo::article::Article, core::core_functions, feed_config::Feed};

#[command] // Tauri decorator
pub fn greet(name: &str) -> String { core_functions::greet(name) }

#[command] // Tauri decorator
pub fn get_articles() -> Vec<Article> { core_functions::get_articles() }

#[command] // Tauri decorator
pub fn load_feeds() -> Vec<Feed> { core_functions::load_feeds() }

#[command] // Tauri decorator
pub fn add_feed(feed_url: String, feed_alias: String, poll_timer: u8) -> Result<(), String> { core_functions::add_feed(feed_url, feed_alias, poll_timer) }
