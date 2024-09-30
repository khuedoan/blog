use crate::posts::{list_posts, PostMetadata};
use askama_axum::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    posts: Vec<PostMetadata>,
}

pub async fn page() -> IndexTemplate {
    IndexTemplate {
        posts: list_posts(),
    }
}
