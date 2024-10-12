use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
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

        if !Path::new(&config_path).exists() {
            let default_config = r#"
[general]
interval = "30m"
mode = "random"
screen_layout = "multi"

[general.wallpapers]
directories = []
extensions = ["jpg", "png", "jpeg"]
recursive = true

[time_of_day]
morning = "/path/to/morning_wallpaper.jpg"
afternoon = "/path/to/afternoon_wallpaper.jpg"
evening = "/path/to/evening_wallpaper.jpg"
night = "/path/to/night_wallpaper.jpg"

[categories]
nature = ["/path/to/nature_wallpapers"]
space = ["/path/to/space_wallpapers"]
tech = ["/path/to/tech_wallpapers"]

[monthly.categories]
winter = [12, 1, 2]
spring = [3, 4, 5]
summer = [6, 7, 8]
fall = [9, 10, 11]

[monthly.wallpapers]
winter = ["/path/to/winter_wallpapers"]
spring = ["/path/to/spring_wallpapers"]
summer = ["/path/to/summer_wallpapers"]
fall = ["/path/to/fall_wallpapers"]

[display]
scaling = "fill"

[logging]
level = 3
file = ""
log_to_file = true
log_to_console = true
            "#;

            let config_path = Path::new(&config_path);
            fs::create_dir_all(config_path.parent().unwrap())
                .expect("Failed to create config directory");

            fs::write(config_path, default_config).expect("Failed to write default config file");
        }

        let config_raw = fs::read_to_string(config_path).expect("Failed to read config file");
        toml::from_str(&config_raw).expect("Failed to parse config file")
    }
}
