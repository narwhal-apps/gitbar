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
