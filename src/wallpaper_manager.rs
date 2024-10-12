use std::{thread::sleep, time::Duration};

use rand::seq::SliceRandom;

use crate::{
    config::{Config, WallpaperConfig},
    file_manager,
    logger::Logger,
};

pub struct WallpaperManager {
    last_index: usize,
}

impl WallpaperManager {
    pub fn new() -> Self {
        WallpaperManager { last_index: 0 }
    }

    pub fn update_index(&mut self) {
        self.last_index += 1;
    }

    pub fn reset_index(&mut self) {
        self.last_index = 0;
    }
}

pub async fn change_wallpaper(config: &Config, wallpaper_manager: &mut WallpaperManager) {
    match config.general.mode.as_str() {
        "random" => {
            select_random_wallpaper(&config.general.wallpapers);
        }
        "sequential" => {
            select_sequential_wallpaper(&config.general.wallpapers, wallpaper_manager);
        }
        _ => {
            Logger::log("Invalid mode", 1);
        }
    }
}

fn select_random_wallpaper(source: &WallpaperConfig) {
    let images = file_manager::collect_images(source);

    let mut thread_rng = rand::thread_rng();
    let image = images.choose(&mut thread_rng).unwrap();

    set_wallpaper(image);
}

fn select_sequential_wallpaper(source: &WallpaperConfig, wallpaper_manager: &mut WallpaperManager) {
    let images = file_manager::collect_images(source);

    let image = images.get(wallpaper_manager.last_index).unwrap();
    wallpaper_manager.update_index();

    if wallpaper_manager.last_index >= images.len() {
        wallpaper_manager.reset_index();
    }

    set_wallpaper(image);
}

fn set_wallpaper(image: &str) {
    Logger::log(&format!("Setting wallpaper: {}", image), 3);
    wallpaper::set_from_path(image).unwrap_or_else(|e| {
        Logger::log(&format!("Failed to set wallpaper: {}", e), 1);
    });
}
