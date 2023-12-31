pub mod game;
pub mod board;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub fn error_message(message: &str) {
    println!("Error message received: {}", message);
}
