use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Config {
    pub general: GeneralConfig,
    pub source: SourceConfig,
    pub time_of_day: Option<HashMap<String, String>>,
    pub categories: Option<HashMap<String, Vec<String>>>,
    pub monthly: Option<MonthlyConfig>,
    pub display: DisplayConfig,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct GeneralConfig {
    pub interval: String,
    pub mode: String,
    pub screen_layout: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct SourceConfig {
    pub directories: Vec<String>,
    pub extensions: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct MonthlyConfig {
    pub categories: HashMap<String, Vec<u32>>,
    pub wallpapers: HashMap<String, Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct DisplayConfig {
    pub scaling: String,
}

impl Config {
    pub fn load() -> Self {
        let config_path = if cfg!(debug_assertions) {
            "config.toml".to_string()
        } else {
            let appdata_path = env::var("APPDATA").expect("APPDATA env variable is not set");
            format!("{}/wallpaperChanger/config.toml", appdata_path)
        };

        let config_raw = fs::read_to_string(config_path).expect("Failed to read config file");
        toml::from_str(&config_raw).expect("Failed to parse config file")
    }
}
