use chrono::{Datelike, Utc};
use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer>
            <div class="mx-auto max-w-7xl overflow-hidden py-20 px-6 sm:py-24 lg:px-8">
                <p class="mt-5 text-center text-sm leading-5 text-gray-500">
                    {format!("© {} Khue Doan. All rights reserved.", Utc::now().year())}
                </p>
            </div>
        </footer>
    }
}
