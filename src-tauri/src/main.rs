// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod internal; // Declare the internal module

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

// TODO: We should add some configuration files so that the menu / port / etc. are customizable via config

fn main() {
    let menu = generate_menu();
    tauri::Builder::default()
        .menu(menu) // Use the built above menu
        // This handles the events that the menu creates
        .on_menu_event(|event| {
            match event.menu_item_id() {
              "quit" => {
                std::process::exit(0);
              }
              "close" => {
                event.window().close().unwrap();
              }
              _ => {}
            }
          })
        // Add all the api end points in the array here
        .invoke_handler(tauri::generate_handler![internal::api::greet, internal::api::add_feed_url])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn generate_menu() -> Menu {
        // Create an internal menu
        // Create a quit and close "option"
        let quit = CustomMenuItem::new("quit".to_string(), "Quit")
            .accelerator("CmdOrCtrl+Q");
        let close = CustomMenuItem::new("close".to_string(), "Close")
            .accelerator("CmdOrCtrl+W");
        // Create the drop down and apply the options to it
        let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
        // Build the menu
        let menu = Menu::new()
            .add_native_item(MenuItem::Copy)
            .add_item(CustomMenuItem::new("hide", "Hide"))
            .add_submenu(submenu);
        return menu;
}

