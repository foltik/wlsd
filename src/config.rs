use anyhow::Result;
use std::{net::SocketAddr, path::PathBuf};

pub async fn load(file: &str) -> Result<Config> {
    let contents = tokio::fs::read_to_string(file).await?;
    Ok(toml::from_str(&contents)?)
}

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub tls: Option<TlsConfig>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ServerConfig {
    pub addr: SocketAddr,
    pub url: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct TlsConfig {
    pub cert: PathBuf,
    pub key: PathBuf,
}
