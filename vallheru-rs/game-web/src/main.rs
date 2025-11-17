mod config;

use anyhow::Result;
use clap::Parser;
use config::AppConfig;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "game-web", about = "Vallheru web server entrypoint")]
struct Cli {
    /// Path to the TOML configuration file
    #[arg(long = "config-path", default_value = "config/game-web.toml")]
    config_path: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = AppConfig::from_path(&cli.config_path)?;
    println!("Loaded configuration:\n{}", config);
    Ok(())
}
