use crate::components::post_list::*;
use crate::components::tag_list::*;
use crate::content::{get_all_tags, get_posts_by_tag};
use leptos::*;
use leptos_router::*;

#[component]
pub fn Tags() -> impl IntoView {
    view! {
        <div class="mx-auto max-w-3xl px-6 lg:px-8">
            <h1 class="mt-2 py-20 text-center text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">
                "Tags"
            </h1>
            <TagList tags=get_all_tags()/>
        </div>
    }
}

#[component]
pub fn Tag() -> impl IntoView {
    let params = use_params_map();
    let tag = move || params().get("id").cloned().unwrap();
    let post_list = move || {
        view! { <PostList posts=get_posts_by_tag(tag())/> }
    };

    view! {
        <h1 class="mt-2 py-20 text-center text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">
            <code>{tag}</code>
        </h1>
        {post_list}
    }
}
