use anyhow::Context;
use axum::Router;
use axum::routing::get;
use std::net as std_net;
use std::sync as std_sync;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;
use tracing::debug;
use tracing::error;
use tracing::info;

use crate::configuration::Configuration;
use crate::frontend::Frontend;
use crate::log::LogManager;

#[derive(Clone)]
pub struct ServerState {
    pub log_manager: std_sync::Arc<LogManager>,
    pub configuration: Configuration,
}

pub async fn start(log_manager: LogManager, configuration: Configuration) {
    let server_state = ServerState {
        log_manager: std_sync::Arc::new(log_manager),
        configuration,
    };
    info!("Starting server...");
    let server_host = &server_state.configuration.server.host;
    let server_port = server_state.configuration.server.port;
    let server_address = format!("{}:{}", server_host, server_port)
        .parse::<std_net::SocketAddr>()
        .context("Failed to parse server address");
    let server_address = match server_address {
        Ok(server_address) => server_address,
        Err(error) => {
            error!("Failed to launch the server: {}", error);
            return;
        }
    };
    debug!("Setting up CORS layer...");
    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    debug!("Setting up router...");
    let router = Router::new()
        .with_state(server_state)
        .layer(cors_layer)
        .fallback(get(Frontend::serve));
    info!("Server listening on {}", server_address);
    axum_server::bind(server_address)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
