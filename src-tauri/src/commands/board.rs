use tauri::State;

use crate::GameManager;

#[tauri::command]
pub fn toggle(game_manager: State<'_, GameManager>, x: usize, y: usize) {
    game_manager
        .mutex
        .lock()
        .unwrap()
        .board
        .toggle(x, y);
}
