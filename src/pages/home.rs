use leptos::*;

use crate::components::intro::*;
use crate::components::post_list::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <Intro />
        <PostList />
    }
}
