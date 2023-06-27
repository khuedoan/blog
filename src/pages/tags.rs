use leptos::*;
use crate::components::tag_list::*;

#[component]
pub fn Tags(cx: Scope) -> impl IntoView {
    view! { cx,
        <TagList />
    }
}
