// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn move_piece(from: &str, to: &str) -> String {
    println!("You moved your piece from {} to {}!", from, to);

    // do some chessing, return new FEN (or same?)
    "r1bqkbnr/pppppppp/2n5/8/8/8/PPPPPPPP/RNBQKBNR".into()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![move_piece])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
