use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub time_of_day: Option<HashMap<String, String>>,
    pub categories: Option<HashMap<String, Vec<String>>>,
    pub monthly: Option<MonthlyConfig>,
    pub display: Option<DisplayConfig>,
    pub logging: LoggingConfig,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GeneralConfig {
    pub interval: String,
    pub mode: String,
    pub screen_layout: String,
    pub wallpapers: WallpaperConfig,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct WallpaperConfig {
    pub directories: Vec<String>,
    pub extensions: Vec<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct MonthlyConfig {
    pub categories: HashMap<String, Vec<u32>>,
    pub wallpapers: HashMap<String, Vec<String>>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct DisplayConfig {
    pub scaling: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, PartialEq, PartialOrd)]
pub enum LogLevel {
    SILENT = 0,
    ERROR = 1,
    WARN = 2,
    INFO = 3,
    DEBUG = 4,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct LoggingConfig {
    pub level: LogLevel,
    pub file: String,
    pub log_to_file: bool,
    pub log_to_console: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            time_of_day: None,
            categories: None,
            monthly: Some(MonthlyConfig::default()),
            display: Some(DisplayConfig::default()),
            logging: LoggingConfig::default(),
        }
    }
}

impl AsRef<Config> for Config {
    fn as_ref(&self) -> &Config {
        self
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            interval: "30m".to_string(),
            mode: "random".to_string(),
            screen_layout: "multi".to_string(),
            wallpapers: WallpaperConfig {
                directories: vec![],
                extensions: vec!["jpg".to_string(), "png".to_string(), "jpeg".to_string()],
            },
        }
    }
}

impl Default for WallpaperConfig {
    fn default() -> Self {
        Self {
            directories: vec![],
            extensions: vec!["jpg".to_string(), "png".to_string(), "jpeg".to_string()],
        }
    }
}

impl Default for MonthlyConfig {
    fn default() -> Self {
        Self {
            categories: HashMap::from([
                ("winter".to_string(), vec![12, 1, 2]),
                ("spring".to_string(), vec![3, 4, 5]),
                ("summer".to_string(), vec![6, 7, 8]),
                ("fall".to_string(), vec![9, 10, 11]),
            ]),
            wallpapers: HashMap::from([
                (
                    "winter".to_string(),
                    vec!["/path/to/winter_wallpapers".to_string()],
                ),
                (
                    "spring".to_string(),
                    vec!["/path/to/spring_wallpapers".to_string()],
                ),
                (
                    "summer".to_string(),
                    vec!["/path/to/summer_wallpapers".to_string()],
                ),
                (
                    "fall".to_string(),
                    vec!["/path/to/fall_wallpapers".to_string()],
                ),
            ]),
        }
    }
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            scaling: "fill".to_string(),
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::ERROR,
            file: "".to_string(),
            log_to_file: false,
            log_to_console: true,
        }
    }
}

// Convert level to string
impl LoggingConfig {
    pub fn level(&self) -> String {
        match self.level {
            LogLevel::SILENT => "SILENT".to_string(),
            LogLevel::ERROR => "ERROR".to_string(),
            LogLevel::WARN => "WARN".to_string(),
            LogLevel::INFO => "INFO".to_string(),
            LogLevel::DEBUG => "DEBUG".to_string(),
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

        if !Path::new(&config_path).exists() {
            let default_config = toml::to_string_pretty(&Config::default())
                .expect("Failed to serialize default config");

            let config_path = Path::new(&config_path);
            fs::create_dir_all(config_path.parent().unwrap())
                .expect("Failed to create config directory");

            fs::write(config_path, default_config).expect("Failed to write default config file");
        }

        let config_raw = fs::read_to_string(config_path).expect("Failed to read config file");
        toml::from_str(&config_raw).expect("Failed to parse config file")
    }
}
