#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod chess;

fn main() {
    tauri::Builder::default()
        .manage(chess::data::PieceLocation(Default::default()))
        .manage(chess::data::Score(Default::default()))
        .manage(chess::data::SelectedSquare(Default::default()))
        .invoke_handler(tauri::generate_handler![
            chess::board::new_game,
            chess::board::get_state,
            chess::board::get_score,
            chess::board::hover_square,
            chess::board::unhover_square,
            chess::board::drop_square,
            chess::board::click_square,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
