use include_dir::{include_dir, Dir};
use std::collections::HashMap;

use gray_matter::engine::YAML;
use gray_matter::Matter;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct PostMetadata {
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub draft: bool,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub title: String,
}

#[derive(Clone)]
pub struct PostData {
    pub path: String,
    pub content: String,
    pub metadata: PostMetadata,
}

pub fn get_all_posts() -> HashMap<String, PostData> {
    static POST_DIR: Dir =
        include_dir!("/home/khuedoan/Documents/worktree/blog/leptos/content/posts");

    POST_DIR
        .files()
        .map(|post| {
            let matter = Matter::<YAML>::new();
            let markdown = matter.parse(post.contents_utf8().unwrap());
            let front_matter: PostMetadata = markdown.data.unwrap().deserialize().unwrap();
            (
                post.path().to_str().unwrap().to_string(),
                PostData {
                    path: post.path().to_str().unwrap().to_string(),
                    metadata: front_matter,
                    content: markdown.content,
                },
            )
        })
        .collect()
}

pub fn get_post(id: String) -> PostData {
    get_all_posts().get(&id).cloned().unwrap()
}
