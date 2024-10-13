use config::{Config, LoggingConfig};
use logger::Logger;
use tokio::time;
use utils::parse_interval;
use wallpaper_manager::{change_wallpaper, WallpaperManager};

mod config;
mod file_manager;
mod logger;
mod utils;
mod wallpaper_manager;

#[tokio::main]
async fn main() {
    let config = Config::load();
    Logger::init(&config.logging).expect("failed to initialize logger");

    let interval = parse_interval(&config.general.interval).expect("invalid interval");

    println!("running with interval: {:?}", interval);
    let mut timer = time::interval(interval);
    let mut wallpaper_manager = WallpaperManager::new();

    loop {
        timer.tick().await;
        let _ = change_wallpaper(&config, &mut wallpaper_manager).await;
        Logger::log("Wallpaper changed", 3);
    }
}
