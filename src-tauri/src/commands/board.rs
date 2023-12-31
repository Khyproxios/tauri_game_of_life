use tauri::State;

use crate::{GameManager, game::Vec2};

#[tauri::command]
pub fn toggle(game_manager: State<'_, GameManager>, position: Vec2) {
    game_manager
        .mutex
        .lock()
        .unwrap()
        .board
        .toggle(position.x, position.y);
}

#[tauri::command]
pub fn resize_board(game_manager: State<'_, GameManager>, width: usize, height: usize) {
    game_manager
        .mutex
        .lock()
        .unwrap()
        .board
        .resize(width, height);
}

#[tauri::command]
pub fn get_alive_count(game_manager: State<'_, GameManager>) -> usize {
    game_manager
        .mutex
        .lock()
        .unwrap()
        .board
        .get_alive_count()
}
