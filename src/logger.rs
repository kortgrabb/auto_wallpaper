use egui::mutex::Mutex;
use lazy_static::lazy_static;

use crate::config::LoggingConfig;

pub struct Logger {
    config: LoggingConfig,
}

impl Logger {
    pub fn init(config: &LoggingConfig) -> Result<(), Box<dyn std::error::Error>> {
        let mut logger = LOGGER.lock();
        logger.config = config.clone(); // Clone here if necessary, or directly use a reference.
        Ok(())
    }

    pub fn log(message: &str, level: u8) {
        let logger = LOGGER.lock();
        if level <= logger.config.level {
            if logger.config.log_to_console {
                println!("[{}] {}", logger.config.level(), message);
            }
            if logger.config.log_to_file {
                // log to file
            }
        }
    }
}

lazy_static! {
    pub static ref LOGGER: Mutex<Logger> = Mutex::new(Logger {
        config: LoggingConfig {
            level: 0,
            file: "".to_string(),
            log_to_file: false,
            log_to_console: true,
        },
    });
}
