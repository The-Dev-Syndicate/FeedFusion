// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod internal; // Declare the internal module
use internal::app_initialization::tauri_init::{
    setup_app, handle_menu_event, generate_menu,
};

fn main() {
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