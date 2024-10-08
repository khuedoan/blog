use crate::page::base;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use maud::PreEscaped;

include!(concat!(env!("OUT_DIR"), "/posts_data.rs"));

#[derive(Debug)]
pub struct PostMetadata {
    pub id: String,
    pub title: String,
    pub date: String,
}

pub fn list_posts() -> Vec<PostMetadata> {
    POSTS
        .iter()
        .map(|&(id, title, date, _content)| PostMetadata {
            id: id.to_string(),
            title: title.to_string(),
            date: date.to_string(),
        })
        .collect()
}

pub fn get_post(path: String) -> Option<(PostMetadata, String)> {
    POSTS
        .iter()
        .filter_map(|(id, title, date, content)| {
            if id == &path {
                Some((
                    PostMetadata {
                        id: id.to_string(),
                        title: title.to_string(),
                        date: date.to_string(),
                    },
                    content.to_string(),
                ))
            } else {
                None
            }
        })
        .next()
}

pub async fn view(Path(id): Path<String>) -> Response {
    match get_post(id) {
        Some((metadata, content)) => base(&metadata.title, PreEscaped(content)).into_response(),
        None => (StatusCode::NOT_FOUND).into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use http_body_util::BodyExt;

    #[tokio::test]
    async fn rendered_content() {
        let response = view(Path("convert-from-init-vim-to-init-lua".to_string())).await;
        assert_eq!(response.status(), StatusCode::OK);

        let body = String::from_utf8(
            response
                .into_body()
                .collect()
                .await
                .unwrap()
                .to_bytes()
                .into(),
        )
        .unwrap();

        assert!(body.contains("<h1>Convert Neovim config from init.vim to init.lua</h1>"));
        assert!(body.contains("<p>Only a few <del>days</del> months left"));
        assert!(body.contains("<h2>Config path</h2>"));
        assert!(body.contains("<p>You can compare my old <a href="));
    }
}
