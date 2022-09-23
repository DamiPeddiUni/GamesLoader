#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{process::Command, os::unix::process::CommandExt};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn open_game(id: i32) {
    let command = Command::new("sh")
    .arg("-c")
    .arg(format!("open steam://run/{}", id)).output();

    match command {
        Ok (e) => {
            println!("{:?}", e)
        },
        Err (e) => {
            println!("{:?}", e)
        }
    }
    
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, open_game])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
