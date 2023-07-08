use crate::components::tag_list::*;
use crate::content::get_all_tags;
use leptos::*;

#[component]
pub fn Tags(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="mx-auto max-w-3xl px-6 lg:px-8">
            <h1 class="mt-2 py-20 text-center text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">
                "Tags"
            </h1>
            <TagList tags=get_all_tags()/>
        </div>
    }
}

#[component]
pub fn Tag(cx: Scope) -> impl IntoView {
    view! { cx, <p>"TODO tag"</p> }
}
