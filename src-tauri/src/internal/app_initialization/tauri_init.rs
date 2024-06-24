use std::time::Duration;
use tauri::Manager;
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

use crate::internal;
use crate::internal::dbo::feed::FeedType;
// use crate::internal::sqlite_db;

pub fn setup_app(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let splashscreen_window = app.get_window("splashscreen").unwrap();
    let main_window = app.get_window("main").unwrap();

    // we perform the initialization code on a new task so the app doesn't freeze
    tauri::async_runtime::spawn(initialize_app(splashscreen_window, main_window));

    let app_handle = app.handle();
    println!("Setting up the app...");

    //###################################################################//
    // TODO: this will come from DB on startup not from requests
    // I want to load all of the feeds from the DB feeds table
    // let feeds = get_the_db_feeds();
    // example to use: https://mastodon.social/@lunar_vagabond.rss
    let feeds = define_feeds();
    // let feeds = internal::sqlite_db::db_fetch_feeds_for_pull().expect("Error grabbing Feeds for BE");
    crate::internal::dbo::feed::start_feed_fetcher(app_handle, feeds);
    //###################################################################//

    // Initialize with DB items
    // let db_feeds = sqlite_db::db_fetch_feed();
    // sqlite_db::fetch_and_emit_feed_db();
    Ok(())
}

async fn initialize_app(
    splashscreen_window: tauri::Window,
    main_window: tauri::Window,
) {
    println!("Initializing...");
    std::thread::sleep(std::time::Duration::from_secs(5)); // FIXME: This is arbitrary time to wait for now but we can do any heavy lifting for DB stuff here
    println!("Done initializing.");

    // After it's done, close the splashscreen and display the main window
    splashscreen_window.close().unwrap();
    main_window.show().unwrap();
}

fn define_feeds() -> Vec<crate::internal::dbo::feed::Feed> {
    vec![
        crate::internal::dbo::feed::Feed {
            url: "https://lorem-rss.herokuapp.com/feed?unit=day".to_string(),
            feed_type: FeedType::RSS,
            poll_interval: Duration::from_secs(24 * 60 * 60), // 24 hours
        },
        crate::internal::dbo::feed::Feed {
            url: "https://run.mocky.io/v3/d3d616ed-4780-41f9-915f-bce277ae0afe".to_string(), // this url may need to be regenerated every so often
            feed_type: FeedType::ATOM,
            poll_interval: Duration::from_secs(20),
        },
        // Add more feeds as needed
    ]
}

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