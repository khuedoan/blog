use askama_axum::Template;
use axum::response::{IntoResponse, Response};

#[derive(Template)]
#[template(path = "about.html")]
pub struct AboutTemplate {}

pub async fn page() -> Response {
    AboutTemplate {}.into_response()
}
