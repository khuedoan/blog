use askama_axum::Template;
use axum::response::{IntoResponse, Response};

#[derive(Template)]
#[template(path = "contact.html")]
pub struct ContactTemplate {}

pub async fn page() -> Response {
    ContactTemplate {}.into_response()
}
