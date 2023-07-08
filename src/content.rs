use gray_matter::engine::YAML;
use gray_matter::Matter;
use include_dir::*;
use pulldown_cmark::{html, Options, Parser};
use serde::Deserialize;
use std::collections::HashMap;

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
    #[serde(default)]
    pub cover: String,
}

#[derive(Clone)]
pub struct PostData {
    pub metadata: PostMetadata,
    pub content: String,
}

pub fn parse_content(post: &str) -> PostData {
    let matter = Matter::<YAML>::new();
    let markdown = matter.parse(post);
    let front_matter: PostMetadata = markdown.data.unwrap().deserialize().unwrap();

    PostData {
        metadata: front_matter,
        content: markdown.content,
    }
}

pub fn markdown_to_html(content: String) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(&content, options);
    let mut html = String::new();
    html::push_html(&mut html, parser);

    html
}

pub fn get_all_posts() -> HashMap<String, PostData> {
    // TODO make this more efficient
    // currently it includes all files in the posts directory at compile time
    static POST_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/content/posts");

    POST_DIR
        .files()
        .map(|post| -> (String, PostData) {
            let matter = Matter::<YAML>::new();
            let markdown = matter.parse(post.contents_utf8().unwrap());
            let front_matter: PostMetadata = markdown.data.unwrap().deserialize().unwrap();

            (
                post.path()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
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
