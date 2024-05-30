// File to house backend logic
use tauri::command;

#[command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[command]
pub fn add_feed_url(feed_url: String, feed_alias: String, poll_timer: u8) {
    println!("[RUST] feed: {} | alias: {} | timer: {}", feed_url, feed_alias, poll_timer);
}