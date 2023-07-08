use crate::components::post_content::*;
use crate::components::post_list::*;
use crate::content::{get_all_posts, get_post};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn Posts(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="text-center">
            <h1 class="mt-2 py-20 text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">
                "Posts"
            </h1>
        </div>
        <PostList posts=get_all_posts()/>
    }
}

#[component]
pub fn Post(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());
    let post = get_post(id());

    view! { cx,
        <Title text=post.metadata.title.clone()/>

        <div class="mx-auto max-w-3xl px-6 lg:px-8">
            <h1 class="mt-2 text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">
                {post.metadata.title}
            </h1>
            <PostContent content=post.content/>
        </div>
    }
}
