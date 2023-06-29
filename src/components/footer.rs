use leptos::*;

#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
    view! { cx,
        <footer>
            <div class="mx-auto max-w-7xl overflow-hidden py-20 px-6 sm:py-24 lg:px-8">
                <p class="mt-5 text-center text-sm leading-5 text-gray-500">
                    "© 2023 Khue Doan. All rights reserved." // TODO do not hard code the year
                </p>
            </div>
        </footer>
    }
}
