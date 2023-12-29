// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use game_of_life::{GameManager, Game, commands};

fn main() {
    tauri::Builder::default()
        .manage(GameManager::new(Game::new()))
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::game::increase,
            commands::game::decrease,
            commands::game::get_value
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
