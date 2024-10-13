use std::fs;

use crate::config::WallpaperConfig;

// Collect all of the images found in the dirs provided in the config
pub fn collect_images(source: &WallpaperConfig) -> Vec<String> {
    let images: Vec<String> = source
        .directories
        .iter()
        .filter_map(|dir| fs::read_dir(dir).ok()) // read the dir, ignore errors
        .flatten()
        .filter_map(|entry| entry.ok()) // Ignore entries that result in errors
        .filter(|entry| entry.path().is_file())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| source.extensions.contains(&ext.to_string()))
                .unwrap_or(false)
        })
        .map(|entry| entry.path().to_string_lossy().into_owned())
        .collect();

    images
}

pub fn get_or_create_file(file_path: &str) -> Result<fs::File, std::io::Error> {
    fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)
}
