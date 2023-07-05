use leptos::*;
use chrono::prelude::*;

#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
    view! { cx,
        <footer>
            <div class="mx-auto max-w-7xl overflow-hidden py-20 px-6 sm:py-24 lg:px-8">
                <p class="mt-5 text-center text-sm leading-5 text-gray-500">
                    {format!("© {} Khue Doan. All rights reserved.", Utc::now().year())}
                </p>
            </div>
        </footer>
    }
}
