use tauri::State;

use crate::{game, GameManager};

#[tauri::command]
pub fn update(game_manager: State<'_, GameManager>) {
    game_manager
        .mutex
        .lock()
        .unwrap()
        .update();
}

#[tauri::command]
pub fn reset(game_manager: State<'_, GameManager>) {
    game_manager
        .mutex
        .lock()
        .unwrap()
        .reset();
}

#[tauri::command]
pub fn get_board(game_manager: State<'_, GameManager>) -> game::Board {
    game_manager
        .mutex
        .lock()
        .unwrap()
        .get_board()
}
