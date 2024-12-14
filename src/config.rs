use anyhow::Result;
use lettre::message::Mailbox;
use std::{net::SocketAddr, path::PathBuf};

pub async fn load(file: &str) -> Result<Config> {
    let contents = tokio::fs::read_to_string(file).await?;
    Ok(toml::from_str(&contents)?)
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Config {
    pub app: ServerConfig,
    pub http: HttpConfig,
    pub https: HttpsConfig,
    pub mail: MailConfig,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct ServerConfig {
    pub url: String,
    pub db: PathBuf,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct HttpConfig {
    pub addr: SocketAddr,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct HttpsConfig {
    pub addr: SocketAddr,
    pub cert: PathBuf,
    pub key: PathBuf,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct MailConfig {
    pub addr: String,
    pub from: Mailbox,
}
