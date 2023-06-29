use leptos::*;

use crate::components::intro::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <Intro />
        // TODO <PostList />
    }
}
