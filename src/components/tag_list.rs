use leptos::*;

#[component]
fn Tag(tag: String) -> impl IntoView {
    view! {
        <a href=format!("/tags/{tag}")>
            <div class="flex flex-1 items-center justify-between truncate rounded-md border border-gray-200 bg-white">
                <div class="flex-1 truncate px-4 py-2 text-sm">
                    <code class="font-medium text-gray-900 hover:text-gray-600">{tag}</code>
                </div>
            </div>
        </a>
    }
}

#[component]
pub fn TagList(tags: Vec<String>) -> impl IntoView {
    view! {
        <ul role="list" class="mt-3 grid grid-cols-1 gap-5 sm:grid-cols-2 sm:gap-6 lg:grid-cols-4">
            {tags
                .iter()
                .map(|tag| {
                    view! {
                        <li key=tag class="col-span-1 rounded-md shadow-sm">
                            <Tag tag=tag.clone()/>
                        </li>
                    }
                })
                .collect_view()}
        </ul>
    }
}
