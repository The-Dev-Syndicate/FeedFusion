// File to house backend logic
use tauri::command;

#[command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[command]
pub fn add_feed_url(feed_url: String) {
    println!("[RUST] Tried to add {}", feed_url);
}