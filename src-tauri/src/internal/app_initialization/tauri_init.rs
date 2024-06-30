use tauri::Manager;
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

use crate::internal;
// use crate::internal::sqlite_db;

pub fn setup_app(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let splashscreen_window = app.get_window("splashscreen").unwrap();
    let main_window = app.get_window("main").unwrap();

    // we perform the initialization code on a new task so the app doesn't freeze
    tauri::async_runtime::spawn(initialize_app(splashscreen_window, main_window));
    // TODO how to gracefully close DB connetions when app closes
    // TODO tauri::async_runtime::spawn(initialize_db()); // insize intialize DB will be while look that looks for new items every 5 min, etc. // THis spawns thread

    let app_handle = app.handle();
    println!("Setting up the app...");

    let feeds = internal::sqlite_db::get_feeds_db().expect("Error grabbing Feeds for BE");
    crate::internal::dbo::feed::start_feed_fetcher(app_handle, feeds);

    Ok(())
}

async fn initialize_app(splashscreen_window: tauri::Window, main_window: tauri::Window) {
    println!("Initializing...");
    std::thread::sleep(std::time::Duration::from_secs(5)); // FIXME: This is arbitrary time to wait for now but we can do any heavy lifting for DB stuff here
    println!("Done initializing.");

    // After it's done, close the splashscreen and display the main window
    splashscreen_window.close().unwrap();
    main_window.show().unwrap();
}

// TODO
// async fn initialize_db() {
//     // While forever
//     // pull from DB
//     // check for duplicates
//     // emit to FE
//     std::thread::sleep(std::time::Duration::from_secs(5)); // FIXME: This is arbitrary time to wait for now but we can do any heavy lifting for DB stuff here
// }

pub fn handle_menu_event(event: tauri::WindowMenuEvent) {
    match event.menu_item_id() {
        "quit" => {
            std::process::exit(0);
        }
        "close" => {
            event.window().close().unwrap();
        }
        _ => {}
    }
}

pub fn generate_menu() -> Menu {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit").accelerator("CmdOrCtrl+Q");
    let close = CustomMenuItem::new("close".to_string(), "Close").accelerator("CmdOrCtrl+W");
    let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
    Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu)
}
