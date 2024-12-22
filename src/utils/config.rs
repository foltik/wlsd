use anyhow::{Context, Result};
use lettre::message::Mailbox;
use std::{net::SocketAddr, path::PathBuf};

impl Config {
    pub async fn load(file: &str) -> Result<Config> {
        async fn load_inner(file: &str) -> Result<Config> {
            let contents = tokio::fs::read_to_string(file).await?;
            Ok(toml::from_str(&contents)?)
        }
        load_inner(file).await.with_context(|| format!("loading config={file}"))
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Config {
    pub app: AppConfig,
    pub net: NetConfig,
    pub acme: Option<AcmeConfig>,
    pub email: EmailConfig,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct AppConfig {
    pub url: String,
    pub db: PathBuf,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct NetConfig {
    pub http_addr: SocketAddr,
    pub https_addr: SocketAddr,
}

/// LetsEncrypt ACME TLS certificate configuration.
#[derive(Clone, Debug, serde::Deserialize)]
pub struct AcmeConfig {
    pub domain: String,
    pub email: String,
    /// Directory where certificates and credentials are stored.
    pub dir: String,
    /// Whether to use the production or staging ACME server.
    pub prod: bool,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct EmailConfig {
    pub addr: String,
    pub from: Mailbox,
}
