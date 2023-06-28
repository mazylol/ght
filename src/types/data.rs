use std::{fs::{File, self}, io::Write};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub api_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Cache {
    pub repos: Vec<Repo>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Repo {
    pub name: String,
    pub owner: String,
}

pub trait FileHandler {
    fn open(filetype: FileType) -> Self;
    fn save(&self, filetype: FileType);
}

pub enum FileType {
    #[allow(dead_code)]
    Config,
    #[allow(dead_code)]
    Cache,
}

impl FileHandler for Config {
    fn open(filetype: FileType) -> Self {
        let file_name = match filetype {
            FileType::Config => "config.json",
            FileType::Cache => "cache.json",
        };

        let path = ProjectDirs::from("com", "mazylol", "ght")
            .unwrap()
            .config_dir()
            .join(file_name);
        let file = File::open(&path);
        if file.is_err() {
            fs::create_dir_all(&path.parent().unwrap()).unwrap();
            let mut file = File::create(&path).unwrap();
            let config = Config {
                api_key: String::from("Replace me"),
            };
            let json = serde_json::to_string_pretty(&config).unwrap();
            file.write_all(json.as_bytes()).unwrap();
            return config;
        } else {
            let config: Config = serde_json::from_reader(file.unwrap()).unwrap();

            if config.api_key == "Replace me" {
                panic!("Please replace the api key in the config file");
            }

            return config;
        }
    }

    fn save(&self, filetype: FileType) {
        let file_name = match filetype {
            FileType::Config => "config.json",
            FileType::Cache => "cache.json",
        };
        
        let path = ProjectDirs::from("com", "mazylol", "ght")
            .unwrap()
            .config_dir()
            .join(file_name);
        let mut file = File::create(&path).unwrap();
        let json = serde_json::to_string_pretty(&self).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }
}
