#![allow(unused)]

use std::{net::SocketAddr, sync::Arc};

use anyhow::{Context, Result};
use axum::{
    extract::{Host, State},
    handler::HandlerWithoutStateExt,
    http::{header, Response, StatusCode, Uri},
    response::{Html, IntoResponse, Redirect},
    routing::get,
    Json, Router,
};
use axum_server::tls_rustls::RustlsConfig;
use tera::Tera;
use tokio::net::TcpListener;

mod config;
use config::*;
use tower_http::services::ServeDir;

#[derive(Clone)]
struct AppState {
    config: Config,
    tera: Tera,
}

fn app(config: Config) -> Result<Router> {
    let state = AppState { config, tera: Tera::new("templates/*")? };
    let router = Router::new()
        .route("/", get(root))
        .nest_service("/assets", ServeDir::new("assets"))
        .with_state(Arc::new(state));
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

    let app = app(config.clone())?.into_make_service();
    println!("Live at {}", &config.app.url);

    match config.https {
        Some(https) => {
            // Redirect HTTP to HTTPS
            tokio::spawn(async move {
                let redirect = move || async move { Redirect::permanent(&config.app.url) };
                axum_server::bind(config.http.addr).serve(redirect.into_make_service()).await
            });

            // Bind HTTPS
            let rustls = RustlsConfig::from_pem_file(https.cert, https.key).await?;
            axum_server::bind_rustls(https.addr, rustls).serve(app).await?;
        }
        None => {
            // Bind HTTP
            axum_server::bind(config.http.addr).serve(app).await?;
        }
    }
    Ok(())
}
