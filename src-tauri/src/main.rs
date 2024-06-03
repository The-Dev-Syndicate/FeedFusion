// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod internal; // Declare the internal module

use std::time::Duration;

use internal::feed::FeedType;
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

// TODO: We should add some configuration files so that the menu / port / etc. are customizable via config

fn main() {
    let menu = generate_menu();
    tauri::Builder::default()
        .setup(|app| {
          let app_handle = app.handle();
          println!("Setting up the app...");
      
          // Define the feeds with their respective poll intervals
          let feeds = vec![
              internal::feed::Feed {
                  url: "https://example.com/rss1".to_string(),
                  feed_type: FeedType::RSS,
                  poll_interval: Duration::from_secs(60),
              },
              internal::feed::Feed {
                  url: "https://example.com/atom1".to_string(),
                  feed_type: FeedType::ATOM,
                  poll_interval: Duration::from_secs(24 * 60 * 60), // 24 hours
              },
              // Add more feeds as needed
          ];
      
          internal::feed::start_feed_fetcher(app_handle, feeds);
          Ok(())
        })
        .menu(menu) // Use the built above menu
        // This handles the events that the menu creates
        .on_menu_event(|event| match event.menu_item_id() {
            "quit" => {
                std::process::exit(0);
            }
            "close" => {
                event.window().close().unwrap();
            }
            _ => {}
        })
        // Add all the api end points in the array here
        .invoke_handler(tauri::generate_handler![
            internal::api::greet,
            internal::api::add_feed,
            internal::api::load_feeds,
            internal::api::get_articles
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn generate_menu() -> Menu {
    // Create an internal menu
    // Create a quit and close "option"
    let quit = CustomMenuItem::new("quit".to_string(), "Quit").accelerator("CmdOrCtrl+Q");
    let close = CustomMenuItem::new("close".to_string(), "Close").accelerator("CmdOrCtrl+W");
    // Create the drop down and apply the options to it
    let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
    // Build the menu
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu);
    return menu;
}
