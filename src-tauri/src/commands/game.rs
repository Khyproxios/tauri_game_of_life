use tauri::State;

use crate::GameManager;

#[tauri::command]
pub fn increase(game_manager: State<'_, GameManager>) {
    game_manager
        .mutex
        .lock()
        .unwrap()
        .increase();
}

#[tauri::command]
pub fn decrease(game_manager: State<'_, GameManager>) {
    game_manager
        .mutex
        .lock()
        .unwrap()
        .decrease();
}

#[tauri::command]
pub fn get_value(game_manager: State<'_, GameManager>) -> usize {
    game_manager
        .mutex
        .lock()
        .unwrap()
        .value
}