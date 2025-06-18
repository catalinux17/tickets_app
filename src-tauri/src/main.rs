#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod ticket;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            ticket::get_all_tickets,
            ticket::create_ticket,
            ticket::remove_last_ticket
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
