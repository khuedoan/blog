use leptos::*;

#[component]
pub fn Intro(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1>
            "Hi there, I’m Khue 👋"
        </h1>
        <p>
            "Welcome to my website, where I write about Linux, DevOps, homelab, workflow optimization, and more!"
        </p>
    }
}
