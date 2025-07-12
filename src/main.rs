mod configuration;
mod log;

use tracing::{error, info};

fn main() {
    let mut log_manager = log::init_log_manager();
    info!("Starting repox...");
    info!("Loading configuration...");
    // let configuration = configuration::init();
    // if configuration.debug {
    //     if let Err(error) = log_manager.enable_debug() {
    //         error!("Failed to enable debug logging: {}", error);
    //     }
    // } else if let Err(error) = log_manager.disable_debug() {
    //     error!("Failed to disable debug logging: {}", error);
    // }
}
