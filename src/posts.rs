use askama_axum::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Deserialize;

static POSTS: &[(&str, &str, &str, &str)] = &[
    (
        "alternatives-to-hashicorp-products",
        "Alternatives to HashiCorp products",
        "2024-04-26T20:11:00+07:00",
        include_str!("../content/posts/alternatives-to-hashicorp-products.md"),
    ),
    (
        "moving-around-efficiently-in-neovim",
        "Moving Around Efficiently in Neovim",
        "2023-06-04T00:02:28+07:00",
        include_str!("../content/posts/moving-around-efficiently-in-neovim.md"),
    ),
    (
        "arch-linux-installation-guide",
        "Arch Linux Installation Guide",
        "2021-05-22T14:29:40+07:00",
        include_str!("../content/posts/arch-linux-installation-guide.md"),
    ),
    (
        "convert-from-init-vim-to-init-lua",
        "Convert Neovim config from init.vim to init.lua",
        "2021-03-12T11:59:12+07:00",
        include_str!("../content/posts/convert-from-init-vim-to-init-lua.md"),
    ),
];

#[derive(Debug, Deserialize)]
pub struct PostMetadata {
    pub id: String,
    pub title: String,
    pub date: String,
}

pub fn list_posts() -> Vec<PostMetadata> {
    POSTS
        .iter()
        .map(|&(id, title, date, _)| PostMetadata {
            id: id.to_string(),
            title: title.to_string(),
            date: date.to_string(),
        })
        .collect()
}

pub fn get_post(path: String) -> Option<(PostMetadata, String)> {
    POSTS
        .iter()
        .find(|(id, _title, _date, _markdown)| id == &path)
        .map(|(id, title, date, markdown)| {
            (
                PostMetadata {
                    id: id.to_string(),
                    title: title.to_string(),
                    date: date.to_string(),
                },
                markdown.to_string(),
            )
        })
}

#[derive(Template)]
#[template(path = "posts/page.html")]
pub struct PostPageTemplate {
    metadata: PostMetadata,
    content: String,
}

pub async fn page(Path(id): Path<String>) -> Response {
    match get_post(id) {
        Some((metadata, content)) => PostPageTemplate { metadata, content }.into_response(),
        None => (StatusCode::NOT_FOUND).into_response(),
    }
}
