use leptos::*;

use crate::components::intro::*;
use crate::components::post_list::*;
use blog::get_all_posts;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <Intro/>
        <PostList posts=get_all_posts()/>
    }
}
