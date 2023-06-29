use crate::components::tag_list::*;
use leptos::*;

#[component]
pub fn Tags(cx: Scope) -> impl IntoView {
    view! { cx,
        <TagList />
    }
}
