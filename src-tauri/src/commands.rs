use std::sync::Mutex;

use crate::{server::AuthServer, utils::get_available_socket_addr};
use tauri::{AppHandle, State, Window};

#[tauri::command]
pub fn start_server(window: Window, state: State<'_, Mutex<AuthServer>>) {
    let mut server = state.lock().unwrap();
    let addr = get_available_socket_addr();
    server.listen(window, addr);
}

#[tauri::command]
pub fn stop_server(state: State<'_, Mutex<AuthServer>>) {
    let mut server = state.lock().unwrap();
    server.stop();
}

#[tauri::command]
pub fn set_review_count(app_handle: AppHandle, count: &str) {
    let tray = app_handle.tray_by_id("gitbar-tray").unwrap();
    let mut rev_count = count.to_string();
    let count_number = count.parse::<i32>().unwrap_or_default();
    if count_number > 0 {
        rev_count.insert_str(0, " ");
    }
    #[cfg(target_os = "macos")]
    tray.set_title(Some(rev_count)).unwrap();
}
