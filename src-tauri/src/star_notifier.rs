use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

pub struct AppState {
    pub username: Option<String>,
    pub token: Option<String>,
    pub period: Option<u32>,
    pub is_running: bool,
    pub worker_handle: Option<thread::JoinHandle<()>>,
    pub stop_sender: Option<mpsc::Sender<()>>, // 通道发送者用于停止线程
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            username: None,
            token: None,
            period: None,
            is_running: false,
            worker_handle: None,
            stop_sender: None,
        }
    }
}

#[tauri::command]
pub fn toggle_state(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    username: String,
    token: String,
    period: u32,
) -> Result<bool, String> {
    let mut app_state = state.lock().unwrap();

    app_state.username = Some(username.clone());
    app_state.token = Some(token.clone());
    app_state.period = Some(period);

    if app_state.is_running {
        // Stop the running thread
        if let Some(stop_sender) = app_state.stop_sender.take() {
            // Send stop signal
            let _ = stop_sender.send(());
        }
        if let Some(handle) = app_state.worker_handle.take() {
            handle.join().unwrap();
        }
        app_state.is_running = false;
    } else {
        // Start a new thread
        let (stop_sender, stop_receiver) = mpsc::channel();
        app_state.stop_sender = Some(stop_sender);

        let period = period;
        let state = Arc::clone(&state);
        let handle = thread::spawn(move || {
            loop {
                {
                    let app_state = state.lock().unwrap();
                    // Perform the task here
                    println!("Processing task for user: {}, token: {}", username, token);
                }

                // Sleep with a timeout to check for stop signal
                if stop_receiver.recv_timeout(Duration::from_secs(period as u64)).is_ok() {
                    break;
                }
            }
        });
        app_state.worker_handle = Some(handle);
        app_state.is_running = true;
    }

    Ok(app_state.is_running)
}
