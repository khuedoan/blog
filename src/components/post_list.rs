use blog::PostData;
use leptos::*;
use std::collections::HashMap;

#[component]
pub fn PostList(cx: Scope, posts: HashMap<String, PostData>) -> impl IntoView {
    view! { cx,
        <ul>
            {posts.iter().map(|(id, post)| {
                view! { cx,
                    <li><a href=format!("/posts/{}", id)>{post.title.to_string()}</a></li>
                }
            }).collect_view(cx)}
        </ul>
    }
}
