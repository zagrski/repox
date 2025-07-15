mod assets;
mod configuration;
mod log;

use std::process as std_process;
use tracing::debug;
use tracing::error;
use tracing::info;

fn main() {
    let mut log_manager = log::init_log_manager();
    info!("Starting repox...");
    info!("Loading configuration...");
    let configuration = match configuration::init() {
        Ok(configuration) => configuration,
        Err(error) => {
            error!("Failed to initialize configuration: {}", error);
            std_process::exit(1);
        }
    };
    if configuration.debug {
        if let Err(error) = log_manager.enable_debug() {
            error!("Failed to enable debug logging: {}", error);
        }
    } else if let Err(error) = log_manager.disable_debug() {
        error!("Failed to disable debug logging: {}", error);
    }
    debug!("Configuration loaded: {:?}", configuration);
}
