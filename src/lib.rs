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
    pub content: String,
    pub metadata: PostMetadata,
}

pub fn get_all_posts() -> HashMap<String, PostData> {
    static POST_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/content/posts");

    POST_DIR
        .files()
        .map(|post| -> (String, PostData) {
            let matter = Matter::<YAML>::new();
            let markdown = matter.parse(post.contents_utf8().unwrap());
            let front_matter: PostMetadata = markdown.data.unwrap().deserialize().unwrap();
            (
                post.path().file_stem().unwrap().to_str().unwrap().to_string(),
                PostData {
                    metadata: front_matter,
                    content: markdown.content,
                },
            )
        })
        .filter(|(_id, post)| !post.metadata.draft) // TODO make this configurable
        .collect()
}

pub fn get_post(id: String) -> PostData {
    get_all_posts().get(&id).cloned().unwrap()
}
