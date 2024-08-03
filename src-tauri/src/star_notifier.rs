

use std::sync::{Arc, Mutex};
use tauri::Manager;

pub struct AppState {
    pub username: Option<String>,
    pub token: Option<String>,
    pub period: Option<u32>,
    pub is_running: bool,
}

#[tauri::command]
pub fn toggle_state(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    username: String,
    token: String,
    period: u32,
) -> Result<bool, String> {
    let mut app_state = state.lock().unwrap();

    app_state.username = Some(username);
    app_state.token = Some(token);
    app_state.period = Some(period);

    app_state.is_running = !app_state.is_running;

    Ok(app_state.is_running)
}

