use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub general: GeneralConfig,
    pub time_of_day: Option<HashMap<String, String>>,
    pub categories: Option<HashMap<String, Vec<String>>>,
    pub monthly: Option<MonthlyConfig>,
    pub display: Option<DisplayConfig>,
    pub logging: Option<LoggingConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GeneralConfig {
    pub interval: String,
    pub mode: String,
    pub screen_layout: String,
    pub wallpapers: WallpaperConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WallpaperConfig {
    pub directories: Vec<String>,
    pub extensions: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MonthlyConfig {
    pub categories: HashMap<String, Vec<u32>>,
    pub wallpapers: HashMap<String, Vec<String>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DisplayConfig {
    pub scaling: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: u8,
    pub file: String,
    pub log_to_file: bool,
    pub log_to_console: bool,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: 3,
            file: "".to_string(),
            log_to_file: false,
            log_to_console: true,
        }
    }
}

// convert level to string
impl LoggingConfig {
    pub fn level(&self) -> String {
        match self.level {
            1 => "ERROR".to_string(),
            2 => "WARN".to_string(),
            3 => "INFO".to_string(),
            4 => "DEBUG".to_string(),
            _ => "UNKNOWN".to_string(),
        }
    }
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
