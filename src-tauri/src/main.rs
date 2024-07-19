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
