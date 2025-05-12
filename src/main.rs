use axum;

use dotenvy::dotenv;
use std::{env, fs::File};
use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
use anyhow::Result;
use tokio::net::TcpListener;

pub mod api;
pub mod routes;
pub mod models;
pub mod dtos;
pub mod services;
pub mod repositories;
pub mod db;
pub mod utils;
pub mod middleware;


#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file
    dotenv().ok();

    let dbpool = db::connect()
        .await;

    // Initialize logging to both stdout and a file
    init_tracing()?;

    // Read host and port from env
    let host = env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = env::var("APP_PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .unwrap_or(3000);
    
    let url = format!("{}:{}", host, port);
    info!("🚀 Starting server at http://{}", url);

    let listener= TcpListener::bind(url)
    .await
    .expect("Address already bound");

   

    // Build Axum app
    let app = routes::routes()
        .layer(axum::Extension(dbpool));

    // Start server
    axum::serve(listener, app).await.expect("Error serving the application");

    Ok(())
}

/// Initialize logging to both stdout and file
fn init_tracing() -> Result<()> {
    let file_appender = File::create("server.log")?;

    let file_layer = fmt::Layer::new()
        .with_writer(file_appender)
        .with_ansi(false)
        .with_target(false)
        .with_level(true);

    let stdout_layer = fmt::Layer::new()
        .with_writer(std::io::stdout)
        .with_ansi(true);

    tracing_subscriber::registry()
        .with(file_layer)
        .with(stdout_layer)
        .with(fmt::Layer::default())
        //.with_filter(tracing_subscriber::EnvFilter::from_default_env()))
        .init();

    Ok(())
}