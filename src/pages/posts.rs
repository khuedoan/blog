use leptos::*;
use crate::components::post_list::*;
use crate::components::post_content::*;
use include_dir::{include_dir, Dir};

pub struct PostData {
    path: String,
    content: String,
}

#[component]
pub fn Posts(cx: Scope) -> impl IntoView {
    view! { cx,
        <PostList />
    }
}

#[component]
pub fn Post(cx: Scope) -> impl IntoView {
    static POST_DIR: Dir = include_dir!("/home/khuedoan/Documents/worktree/blog/leptos/content/posts");
    let mut posts: Vec<PostData> = Vec::new();

    for file in POST_DIR.files() {
        let post = PostData {
            path: file.path().to_str().unwrap().to_string(),
            content: file.contents_utf8().unwrap().to_string(),
        };

        posts.push(post);
    };

    view! { cx,
        <PostContent content=posts[0].content.to_string()/>
    }
}
