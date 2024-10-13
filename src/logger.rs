use std::sync::Mutex;

use lazy_static::lazy_static;
use std::io::Write;

use crate::{
    config::{LogLevel, LoggingConfig},
    file_manager,
};

pub struct Logger {
    config: LoggingConfig,
}

impl Logger {
    pub fn init(config: &LoggingConfig) -> Result<(), Box<dyn std::error::Error>> {
        let mut logger = LOGGER.lock().unwrap();
        logger.config = config.clone();

        Ok(())
    }

    pub fn log(message: &str, level: LogLevel) {
        let logger = LOGGER.lock().unwrap();
        if level <= logger.config.level {
            if logger.config.log_to_console {
                println!("[{}] {}", logger.config.level(), message);
            }
            if logger.config.log_to_file {
                // TODO: implement file logging
                if logger.config.log_to_file {
                    let log_file_name = &logger.config.file;
                    let file = file_manager::get_or_create_file(log_file_name);

                    match file {
                        Ok(f) => {
                            let date_format = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");

                            let mut file = f;
                            if let Err(e) = writeln!(
                                file,
                                "{} - [{}] {}",
                                date_format,
                                logger.config.level(),
                                message
                            ) {
                                eprintln!("Couldn't write to file: {}", e);
                            }
                        }
                        Err(e) => {
                            Logger::log(
                                &format!("error reading or creating log file: {}", e),
                                LogLevel::ERROR,
                            );
                        }
                    }
                }
            }
        }
    }
}

lazy_static! {
    pub static ref LOGGER: Mutex<Logger> = Mutex::new(Logger {
        config: LoggingConfig {
            level: crate::config::LogLevel::SILENT,
            file: "".to_string(),
            log_to_file: false,
            log_to_console: true,
        },
    });
}
