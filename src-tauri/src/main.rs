// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod star_notifier;
mod file_store;

use tauri::{SystemTray,CustomMenuItem, SystemTrayMenu,SystemTrayEvent};
use tauri::Manager;
use std::sync::{Arc, Mutex};


fn main() {


    let state = Arc::new(Mutex::new(star_notifier::AppState::new()));

    let quit: CustomMenuItem = CustomMenuItem::new("quit".to_string(), "退出");
    let star_notifier = CustomMenuItem::new("StarNotifier".to_string(), "github star通知器");
    let tray_menu = SystemTrayMenu::new()
    .add_item(star_notifier)
    .add_item(quit);

    let system_tray = SystemTray::new()
      .with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(system_tray)
        .manage(state)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
              match id.as_str() {
                "StarNotifier" => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                    window.set_focus().unwrap();

                    match file_store::load() {
                      Ok(file_data) => {
                        window.emit("file-data", file_data).unwrap();
                      }
                      Err(e) => {
                          eprintln!("Failed to read file data: {}", e);
                      }
                  }
                }
                "quit" => {
                    /*let main_window = app.get_window("main").unwrap();
                    match file_store::load() {
                        Ok(file_data) => {
                            main_window.emit("file-data", file_data).unwrap();
                        }
                        Err(e) => {
                            eprintln!("Failed to read file data: {}", e);
                        }
                    }*/
                    std::process::exit(0);
                }
                _ => {}
              }
            }
            _ => {}
          })
        .invoke_handler(tauri::generate_handler![star_notifier::toggle_state])
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let app_handle = app.handle();
            app.manage(app_handle.clone());

            Ok(())
        }).run(tauri::generate_context!())
        .expect("error while running tauri application");
}
