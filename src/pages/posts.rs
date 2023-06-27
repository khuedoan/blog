use leptos::*;
use crate::components::post_list::*;
use crate::components::post_content::*;

#[component]
pub fn Posts(cx: Scope) -> impl IntoView {
    view! { cx,
        <PostList />
    }
}

#[component]
pub fn Post(cx: Scope) -> impl IntoView {
    view! { cx,
        <PostContent />
    }
}
