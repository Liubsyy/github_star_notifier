use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;
use reqwest::{Error,header};
use serde::Deserialize;

use crate::file_store;
use tokio::runtime::Runtime;


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

#[derive(Deserialize, Debug)]
struct Repo {
    full_name: String,
    stargazers_count: u32,
    forks_count: u32,
}

async fn fetch_repos(username: &str, token: &str) -> Result<Vec<Repo>, Error> {
    let client = reqwest::Client::new();
    let mut all_repos = Vec::new();
    let mut page = 1;

    loop {
        let url = format!("https://api.github.com/users/{}/repos?page={}&per_page=100", username, page);
        let request = client
            .get(&url)
            .header("User-Agent", "request");

        let request = if !token.is_empty() {
            request.header(header::AUTHORIZATION, format!("token {}", token))
        } else {
            request
        };

        let response = request.send().await?;
        
        if !response.status().is_success() {
            eprintln!("Error fetching repos: {}", response.status());
            break;
        }

        let repos: Vec<Repo> = response.json().await?;
        
        if repos.is_empty() {
            break;
        }

        all_repos.extend(repos);
        page += 1;
    }

    Ok(all_repos)
}

#[tauri::command]
pub fn toggle_state(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    app_handle: tauri::AppHandle,
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
     

            let rt = Runtime::new().unwrap();
            loop {
                {
               
                    let app_state = state.lock().unwrap();
                    
                    //查询github接口获取数据
                    let username = app_state.username.clone().unwrap();
                    let token = app_state.token.clone().unwrap();
                    let period: u32 = app_state.period.clone().unwrap();

                    // Perform the task here
                    println!("Processing task for user: {}, token: {}", username, token);

                    // 查询github接口获取数据
                    let repos = rt.block_on(fetch_repos(&username, &token)).unwrap();
              
                    //和上一次文件数据进行差值比对
                    match file_store::load() {
                        Ok(last_star_info) => {
                            for repo in &repos {
                                if let Some(last_info) = last_star_info.projects.iter().find(|p| p.project_name == repo.full_name) {
                                    let star_increasement = repo.stargazers_count as i32 - last_info.star as i32;
                                    let fork_increasement = repo.forks_count as i32 - last_info.fork as i32;
                                    
                                    if star_increasement!=0 || fork_increasement!=0 {
                                        let star_message = if star_increasement > 0 {
                                            format!("(+{})", star_increasement)
                                        } else if star_increasement < 0 {
                                            format!("({})", star_increasement)
                                        } else {
                                            String::new()
                                        };
                                    
                                    
                                        let fork_message = if fork_increasement > 0 {
                                            format!("(+{})", fork_increasement)
                                        } else if fork_increasement < 0 {
                                            format!("({})", fork_increasement)
                                        } else {
                                            String::new()
                                        };
                                    
                                        // 打印差值
                                        println!(
                                            "[{}] Stars: {}{}, Forks: {}{}",
                                            repo.full_name, repo.stargazers_count, star_message, repo.forks_count, fork_message
                                        );
                                    
                                        // 通知
                                        tauri::api::notification::Notification::new(&app_handle.config().tauri.bundle.identifier)
                                        .title("Github通知")
                                        .body(&format!(
                                            "[{}] Stars: {}{}, Forks: {}{}",
                                             repo.full_name, repo.stargazers_count, star_message, repo.forks_count, fork_message
                                         ))
                                        .show()
                                        .unwrap();
                                       
                                        // notify_rust::Notification::new()
                                        //     .summary("Github通知")
                                        //     .body(&format!(
                                        //        "[{}] Stars: {}{}, Forks: {}{}",
                                        //         repo.full_name, repo.stargazers_count, star_message, repo.forks_count, fork_message
                                        //     ))
                                        //     .show()
                                        //     .unwrap();
                                    }
                                } else {
                                    println!(
                                        "Repo: {}, Stars: {}, Forks: {}",
                                        repo.full_name, repo.stargazers_count, repo.forks_count
                                    );
                                }
                            }
                        },
                        Err(e) => {
                            eprintln!("Failed to load last star info: {}", e);
                        }
                    }

                    //写入结果到文件
                    let mut star_info = file_store::StarInfo::new(username.clone(), token.clone(), period);
                    for repo in &repos {
                        star_info.add_project(repo.full_name.clone(), repo.stargazers_count, repo.forks_count);
                    }
                    if let Err(e) = file_store::save(&star_info) {
                        eprintln!("Failed to save star info: {}", e);
                    }
                    


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

