use anyhow::{Context, Result};
use axum::{handler::HandlerWithoutStateExt as _, response::Redirect};
use axum_server::tls_rustls::RustlsConfig;
use mimalloc::MiMalloc;
use tera::Tera;

mod app;
mod config;
mod db;
mod mail;

use config::*;
use db::Db;
use mail::Mail;

#[global_allocator]
static ALLOC: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();
    let file = std::env::args().nth(1).context("usage: wlsd <config.yaml>")?;
    let config = config::load(&file).await.with_context(|| format!("loading config={file}"))?;

    let app = app::build(config.clone()).await?.into_make_service();
    tracing::info!("Live at {}", &config.app.url);

    // Redirect HTTP to HTTPS
    tokio::spawn(async move {
        let redirect = move || async move { Redirect::permanent(&config.app.url) };
        axum_server::bind(config.http.addr).serve(redirect.into_make_service()).await
    });

    // Bind HTTPS
    let rustls = RustlsConfig::from_pem_file(config.https.cert, config.https.key).await?;
    axum_server::bind_rustls(config.https.addr, rustls).serve(app).await?;
    Ok(())
}
