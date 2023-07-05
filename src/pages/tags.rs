use crate::components::tag_list::*;
use leptos::*;

#[component]
pub fn Tags(cx: Scope) -> impl IntoView {
    view! { cx, <TagList/> }
}

#[component]
pub fn Tag(cx: Scope) -> impl IntoView {
    view! { cx, <p>"TODO tag"</p> }
}
