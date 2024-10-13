use std::{io::Read, sync::Mutex};

use lazy_static::lazy_static;

use crate::config::LoggingConfig;

pub struct Logger {
    config: LoggingConfig,
}

impl Logger {
    pub fn init(config: &LoggingConfig) -> Result<(), Box<dyn std::error::Error>> {
        let mut logger = LOGGER.lock().unwrap();
        logger.config = config.clone();

        Ok(())
    }

    pub fn log(message: &str, level: u8) {
        let logger = LOGGER.lock().unwrap();
        if level <= logger.config.level {
            if logger.config.log_to_console {
                println!("[{}] {}", level, message);
            }
            if logger.config.log_to_file {
                // TODO: implement file logging
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
