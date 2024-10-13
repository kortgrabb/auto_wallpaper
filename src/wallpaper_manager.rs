use crate::{
    config::{Config, WallpaperConfig},
    file_manager,
    logger::Logger,
};
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref IMAGE_CACHE: Mutex<HashMap<String, Vec<String>>> = Mutex::new(HashMap::new());
}

pub struct WallpaperManager {
    last_index: usize,
}

impl WallpaperManager {
    pub fn new() -> Self {
        Self { last_index: 0 }
    }

    pub fn update_index(&mut self, max_index: usize) {
        self.last_index = (self.last_index + 1) % max_index;
    }

    pub fn reset_index(&mut self) {
        self.last_index = 0;
    }
}

pub async fn change_wallpaper(
    config: &Config,
    wallpaper_manager: &mut WallpaperManager,
) -> Result<(), String> {
    match config.general.mode.as_str() {
        "random" => select_random_wallpaper(&config.general.wallpapers)?,
        "sequential" => select_sequential_wallpaper(&config.general.wallpapers, wallpaper_manager)?,
        _ => Logger::log("Invalid mode", 1),
    }
    Ok(())
}

// This function is used to select a random image from the list
fn select_random_wallpaper(source: &WallpaperConfig) -> Result<(), String> {
    let images = get_cached_images(source);
    let mut rng = rand::thread_rng();
    if let Some(image) = images.choose(&mut rng) {
        set_wallpaper(image)?;
        Ok(())
    } else {
        Logger::log("No images found", 1);
        Err("No images found".to_string())
    }
}

// This function is used to select the next image in the sequence
fn select_sequential_wallpaper(
    source: &WallpaperConfig,
    wallpaper_manager: &mut WallpaperManager,
) -> Result<(), String> {
    let images = get_cached_images(source);
    if let Some(image) = images.get(wallpaper_manager.last_index) {
        set_wallpaper(image)?;
        wallpaper_manager.update_index(images.len());
        Ok(())
    } else {
        Logger::log("No images found", 1);
        Err("No images found".to_string())
    }
}

// This function is used to cache the images in memory to avoid reading the filesystem every time
fn get_cached_images(source: &WallpaperConfig) -> Vec<String> {
    let mut cache = IMAGE_CACHE.lock().unwrap();
    let dirs = source.directories.clone();

    if cache.is_empty() {
        let images = file_manager::collect_images(source);
        for dir in &dirs {
            cache.insert(dir.clone(), images.clone());
        }
    }

    let mut images = Vec::new();
    for dir in &dirs {
        if let Some(dir_images) = cache.get(dir) {
            images.extend(dir_images.clone());
        }
    }
    images
}

fn set_wallpaper(image: &str) -> Result<(), String> {
    Logger::log(&format!("Setting wallpaper: {}", image), 3);
    if let Err(e) = wallpaper::set_from_path(image) {
        Logger::log(&format!("Error setting wallpaper: {}", e), 1);
        Err(e.to_string())
    } else {
        Ok(())
    }
}
