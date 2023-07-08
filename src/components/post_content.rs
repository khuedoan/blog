use crate::content::markdown_to_html;
use leptos::*;

#[component]
pub fn PostContent(cx: Scope, content: String) -> impl IntoView {
    view! { cx, <div class="prose max-w-none" inner_html=markdown_to_html(content)></div> }
}
