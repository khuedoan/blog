use leptos::*;

#[component]
pub fn Intro(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="text-center">
            <h1 class="text-4xl font-bold tracking-tight text-gray-900 sm:text-6xl">
                "Hi there, I’m Khue 👋"
            </h1>
            <p class="mt-6 text-lg leading-8 text-gray-600">
                "Welcome to my website, where I write about Linux, DevOps, homelab, workflow optimization, and more!"
            </p>
        </div>
    }
}
