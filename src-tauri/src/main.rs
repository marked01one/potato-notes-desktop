// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use crate::commands::calculator;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn morning(_message: String) -> i32 {
    let mut stack: Vec<i32> = Vec::new();

    stack.push(1);

    let result = calculator::calculator("Hello!");

    return result
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            morning
        ])


        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
