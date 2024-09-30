use askama_axum::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

pub async fn page() -> IndexTemplate {
    IndexTemplate {}
}
