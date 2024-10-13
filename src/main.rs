#![forbid(unsafe_code)]

use axum::{routing::get, Router};
use tokio::signal;
use tower_http::compression::CompressionLayer;

mod about;
mod contact;
mod index;
mod page;
mod posts;
mod public;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    let app = app();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

fn app() -> Router {
    Router::new()
        .route("/", get(index::view))
        .route("/posts/:id", get(posts::view))
        .route("/about", get(about::view))
        .route("/contact", get(contact::view))
        .route("/*path", get(public::file).layer(CompressionLayer::new()))
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
