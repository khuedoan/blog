#![forbid(unsafe_code)]

use axum::{routing::get, Router};

mod index;
mod posts;
mod public;

#[tokio::main]
async fn main() {
    let app = app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn app() -> Router {
    Router::new()
        .route("/", get(index::page))
        .route("/posts/:id", get(posts::page))
        .route("/:file", get(public::file))
}
