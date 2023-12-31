// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use game_of_life::{GameManager, Game, commands};

fn main() {
    tauri::Builder::default()
        .manage(GameManager::new(Game::new(10, 10)))
        .invoke_handler(tauri::generate_handler![
            commands::error_message,
            commands::game::update,
            commands::game::reset,
            commands::game::get_board,
            commands::board::toggle,
            commands::board::resize_board,
            commands::board::get_alive_count
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
