mod config;

use anyhow::{Context, Result};
use axum::{Json, Router, routing::get};
use clap::Parser;
use config::AppConfig;
use serde::Serialize;
use std::path::PathBuf;
use tokio::net::TcpListener;

#[derive(Parser, Debug)]
#[command(name = "game-web", about = "Vallheru web server entrypoint")]
struct Cli {
    /// Path to the TOML configuration file
    #[arg(long = "config-path", default_value = "config/game-web.toml")]
    config_path: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = AppConfig::from_path(&cli.config_path)?;
    println!("Loaded configuration:\n{}", config);

    let listener = TcpListener::bind((config.server.host.as_str(), config.server.port))
        .await
        .with_context(|| {
            format!(
                "Could not bind HTTP listener on {}:{}",
                config.server.host, config.server.port
            )
        })?;
    println!(
        "Listening on http://{}:{}",
        config.server.host, config.server.port
    );

    let app = Router::new().route("/health", get(health_handler));

    axum::serve(listener, app)
        .await
        .context("HTTP server terminated unexpectedly")?;

    Ok(())
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

async fn health_handler() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}
