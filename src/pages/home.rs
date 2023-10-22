use leptos::*;

use crate::components::intro::*;
use crate::components::post_list::*;
use crate::content::get_all_posts;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Intro/>
        <PostList posts=get_all_posts()/>
    }
}
