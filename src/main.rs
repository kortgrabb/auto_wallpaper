use config::Config;

mod config;
mod utils;
mod wallpaper_manager;

#[tokio::main]
async fn main() {
    let config = Config::load();
}
