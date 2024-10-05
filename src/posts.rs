use askama_axum::Template;
use axum::extract::Path;
use axum::response::IntoResponse;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PostMetadata {
    pub id: String,
    pub title: String,
    pub draft: bool,
    pub cover: String,
}

#[derive(Debug, Deserialize)]
pub struct Post {
    pub metadata: PostMetadata,
    pub content: String,
}

pub fn list_posts() -> Vec<PostMetadata> {
    let posts = vec![PostMetadata {
        id: "postidfoo".to_string(),
        title: "posttitle foo".to_string(),
        draft: false,
        cover: "foo".to_string(),
    }];

    posts
}

pub fn get_post(id: String) -> Post {
    Post {
        metadata: PostMetadata {
            id,
            title: "posttitle foo".to_string(),
            draft: false,
            cover: "foo".to_string(),
        },
        content: "foocontent".to_string(),
    }
}

#[derive(Template)]
#[template(path = "posts/page.html")]
pub struct PostPageTemplate {
    post: Post,
}

pub async fn page(Path(id): Path<String>) -> impl IntoResponse {
    PostPageTemplate { post: get_post(id) }
}
