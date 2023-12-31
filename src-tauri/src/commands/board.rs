use tauri::State;

use crate::{GameManager};

#[derive(serde::Deserialize)]
pub struct Vec2 {
    pub x: usize,
    pub y: usize
}

#[tauri::command(rename_all = "snake_case")]
pub fn toggle(game_manager: State<'_, GameManager>, position: Vec2) {
    println!("Toggling Cell at <{}, {}>", position.x, position.y);

    game_manager
        .mutex
        .lock()
        .unwrap()
        .board
        .toggle(position.x, position.y);
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
