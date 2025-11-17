use std::fmt;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub templates: TemplateConfig,
    #[serde(default)]
    pub features: FeatureToggles,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,
    pub port: u16,
    #[serde(default = "default_workers")]
    pub workers: usize,
    #[serde(default = "default_request_timeout_secs")]
    pub request_timeout_secs: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    #[serde(default = "default_pool_size")]
    pub pool_size: u32,
    #[serde(default = "default_connect_timeout_secs")]
    pub connect_timeout_secs: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TemplateConfig {
    pub template_root: PathBuf,
    #[serde(default)]
    pub hot_reload: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FeatureToggles {
    #[serde(default)]
    pub enable_registration: bool,
    #[serde(default)]
    pub enable_world_map: bool,
}

impl Default for FeatureToggles {
    fn default() -> Self {
        Self {
            enable_registration: true,
            enable_world_map: true,
        }
    }
}

impl AppConfig {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Could not read configuration file: {}", path.display()))?;
        let config = toml::from_str::<AppConfig>(&content)
            .with_context(|| format!("Failed to parse configuration file: {}", path.display()))?;
        Ok(config)
    }
}

impl fmt::Display for AppConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Server: {}", self.server)?;
        writeln!(f, "Database: {}", self.database)?;
        writeln!(f, "Templates: {}", self.templates)?;
        write!(f, "Features: {}", self.features)
    }
}

impl fmt::Display for ServerConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "host={} port={} workers={} request_timeout_secs={}",
            self.host, self.port, self.workers, self.request_timeout_secs
        )
    }
}

impl fmt::Display for DatabaseConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "url={} pool_size={} connect_timeout_secs={}",
            self.url, self.pool_size, self.connect_timeout_secs
        )
    }
}

impl fmt::Display for TemplateConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "template_root={} hot_reload={}",
            self.template_root.display(),
            self.hot_reload
        )
    }
}

impl fmt::Display for FeatureToggles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "enable_registration={} enable_world_map={}",
            self.enable_registration, self.enable_world_map
        )
    }
}

fn default_host() -> String {
    "127.0.0.1".to_string()
}

fn default_workers() -> usize {
    4
}

fn default_request_timeout_secs() -> u64 {
    30
}

fn default_pool_size() -> u32 {
    10
}

fn default_connect_timeout_secs() -> u64 {
    5
}
