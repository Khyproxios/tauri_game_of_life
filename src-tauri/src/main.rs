// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use game_of_life::{GameManager, Game, commands};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                // window.open_devtools();
                // window.close_devtools();
            }
            Ok(())
        })
        .manage(GameManager::new(Game::new(10, 10)))
        .invoke_handler(tauri::generate_handler![
            commands::error_message,
            commands::greet,
            commands::game::update,
            commands::game::reset,
            commands::game::get_board,
            commands::board::toggle,
            commands::board::get_alive_count
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
