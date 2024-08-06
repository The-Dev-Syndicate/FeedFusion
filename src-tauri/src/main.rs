// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod internal;

use env_logger::Env;

// Declare the internal module
use internal::app_initialization::tauri_init::{generate_menu, handle_menu_event, setup_app};

fn main() {
    // Run at a different level with RUST_LOG=debug cargo tauri dev  for debug level. See: https://crates.io/crates/env_logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let menu = generate_menu();

    // create sqliteDB -- including tables, generate fake data
    // TODO only do this once in install of app
    internal::sqlite_db::create_db();
    internal::sqlite_db::create_fake_data();
    
    // For testing purposes, to see that feeds have all information expected
    let full_feeds = internal::sqlite_db::get_feeds_db().expect("No feeds found on startup");
    let feed_items = internal::sqlite_db::get_feed_items_db(); //.expect("No feed items found on startup");

    println!("FEEDS\n");
    for feed in full_feeds {
        println!("Feed: {:?}", feed)
    }

    println!("\nFEED ITEMS\n");
    if feed_items.len() < 1 {
        print!("No feed items found on startup");
    }
    else {
        for entry in feed_items {
            print!("Entry: {:?}\n\n", entry)
        }
    }
    
    tauri::Builder::default()
        .setup(setup_app)
        .menu(menu)
        .on_menu_event(handle_menu_event)
        .invoke_handler(tauri::generate_handler![
            internal::api::endpoints::greet,
            internal::api::endpoints::add_feed,
            internal::api::endpoints::load_feeds,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
