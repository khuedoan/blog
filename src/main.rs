#![forbid(unsafe_code)]

use axum::{routing::get, Router};
use tokio::signal;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::EnvFilter;

mod about;
mod contact;
mod index;
mod page;
mod posts;
mod public;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_env("LOG_LEVEL").unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .without_time()
        .init();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

fn app() -> Router {
    Router::new()
        .route("/", get(index::view))
        .route("/posts/{id}", get(posts::view))
        .route("/about", get(about::view))
        .route("/contact", get(contact::view))
        .route("/{*path}", get(public::file))
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
