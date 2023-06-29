use crate::components::post_content::*;
use crate::components::post_list::*;
use blog::{get_all_posts, get_post};
use leptos::*;
use leptos_router::*;

#[component]
pub fn Posts(cx: Scope) -> impl IntoView {
    view! { cx,
        <PostList posts=get_all_posts()/>
    }
}

#[component]
pub fn Post(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());

    view! { cx,
        <PostContent content=get_post(id()).content.to_string()/>
    }
}
