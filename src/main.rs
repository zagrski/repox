mod access_token;
mod assets;
mod configuration;
mod frontend;
mod log;
mod server;
mod user;

use tracing::debug;
use tracing::error;
use tracing::info;

#[tokio::main]
async fn main() {
    let mut log_manager = log::init_log_manager();
    info!("Starting repox...");
    info!("Initializing configuration...");
    let configuration = match configuration::init() {
        Ok(configuration) => configuration,
        Err(error) => {
            error!("Failed to initialize configuration: {}", error);
            return;
        }
    };
    if configuration.debug {
        if let Err(error) = log_manager.enable_debug() {
            error!("Failed to enable debug logging: {}", error);
        }
    } else if let Err(error) = log_manager.disable_debug() {
        error!("Failed to disable debug logging: {}", error);
    }
    debug!("Initialized configuration: {:?}", configuration);
    server::start(log_manager, configuration).await;
}
