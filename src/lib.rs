use include_dir::{include_dir, Dir};
use std::collections::HashMap;

#[derive(Clone)]
pub struct PostData {
    pub content: String,
    pub date: String,
    pub draft: bool,
    pub path: String,
    pub summary: String,
    pub tags: Vec<String>,
    pub title: String,
}

pub fn get_all_posts() -> HashMap<String, PostData> {
    static POST_DIR: Dir =
        include_dir!("/home/khuedoan/Documents/worktree/blog/leptos/content/posts");

    POST_DIR
        .files()
        .map(|post| {
            (
                post.path().to_str().unwrap().to_string(),
                PostData {
                    content: post.contents_utf8().unwrap().to_string(),
                    date: "".to_string(),
                    draft: false,
                    path: post.path().to_str().unwrap().to_string(),
                    summary: "".to_string(),
                    tags: Vec::new(),
                    title: post.path().to_str().unwrap().to_string(), // TODO
                },
            )
        })
        .collect()
}

pub fn get_post(id: String) -> PostData {
    get_all_posts().get(&id).cloned().unwrap()
}
