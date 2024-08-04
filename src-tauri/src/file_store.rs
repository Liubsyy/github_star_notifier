use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StarInfo {
    pub username: String,
    pub token: String,
    pub period: u32,
    pub projects: Vec<Project>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    pub project_name: String,
    pub star: u32,
    pub fork: u32,
}

impl StarInfo {
    pub fn new(username: String, token: String, period: u32) -> Self {
        StarInfo {
            username,
            token,
            period,
            projects: Vec::new(),
        }
    }

    pub fn add_project(&mut self, project_name: String, star: u32, fork: u32) {
        self.projects.push(Project {
            project_name,
            star,
            fork,
        });
    }

    fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let json = serde_json::to_string_pretty(&self)?;
        let mut file = File::create(filename)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
    fn load_from_file(filename: &str) -> io::Result<Self> {
        match File::open(filename) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                let app_state: StarInfo = serde_json::from_str(&contents)?;
                Ok(app_state)
            }
            Err(error) => {
                if error.kind() == io::ErrorKind::NotFound {
                    // 文件不存在时创建一个新的文件并写入默认内容
                    let starInfo: StarInfo = StarInfo::new("".to_string(), "".to_string(), 120);
                    starInfo.save_to_file(filename)?;
                    Ok(starInfo)
                } else {
                    // 其他错误
                    Err(error)
                }
            }
        }
    }
}


pub fn save(star_info: &StarInfo) -> io::Result<()> {
    star_info.save_to_file("github_star.txt")
}

pub fn load() -> io::Result<StarInfo> {
    StarInfo::load_from_file("github_star.txt")
}


pub fn write_demo() -> io::Result<()> {
    // 创建一个新的StarInfo对象
    let mut app_state = StarInfo::new("user1".to_string(), "token123".to_string(), 30);

    // 添加一些项目
    app_state.add_project("Project1".to_string(), 101, 50);
    app_state.add_project("Project2".to_string(), 201, 75);

    // 保存到文件
    app_state.save_to_file("app_state.json")?;
    
    // 从文件读取
    let loaded_app_state = StarInfo::load_from_file("app_state.json")?;

    // 打印读取的内容
    println!("{:?}", loaded_app_state);

    Ok(())
}
