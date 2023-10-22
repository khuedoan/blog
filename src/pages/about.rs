use crate::components::post_content::*;
use leptos::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <div class="mx-auto max-w-3xl px-6 lg:px-8">
            <h1 class="mt-2 py-20 text-center text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">
                "About"
            </h1>
            <PostContent content=include_str!("../../content/about.md").to_string()/>
        </div>
    }
}
