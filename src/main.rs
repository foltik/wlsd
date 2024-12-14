#![allow(unused)]

use std::sync::Arc;

use anyhow::{Context, Result};
use axum::{
    extract::State,
    http::Response,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use axum_server::tls_rustls::RustlsConfig;
use tera::Tera;
use tokio::net::TcpListener;

mod config;
use config::*;

#[derive(Clone)]
struct AppState {
    tera: Tera,
}

fn app() -> Result<Router> {
    let state = AppState { tera: Tera::new("assets/*")? };
    let router = Router::new().route("/", get(root)).with_state(Arc::new(state));
    Ok(router)
}

async fn root(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let mut ctx = tera::Context::new();
    ctx.insert("message", "Hello, world!");

    let html = state.tera.render("wlsd.tera.html", &ctx).unwrap();
    Html(html).into_response()
}

#[tokio::main]
async fn main() -> Result<()> {
    let file = std::env::args().nth(1).context("usage: wlsd <config.yaml>")?;
    let config = config::load(&file).await.with_context(|| format!("loading config={file}"))?;
    println!("Running with {config:#?}");

    let app = app()?.into_make_service();
    let ServerConfig { addr, url } = config.server;

    println!("Listening at {url}");
    match config.tls {
        Some(tls) => {
            let rustls = RustlsConfig::from_pem_file(tls.cert, tls.key).await?;
            axum_server::bind_rustls(addr, rustls).serve(app).await?;
        }
        None => {
            axum_server::bind(addr).serve(app).await?;
        }
    }

    Ok(())
}
