// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// This is how we can create things for react to call on the backend
// It is an extermely simple command that at the end of the day simple returns hello world
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
