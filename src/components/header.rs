use leptos::*;

#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    view! { cx,
        <header>
            <nav>
                <a href="/">
                    "Khue Doan"
                </a>
                <a href="/about">
                    "About"
                </a>
                <a href="/posts">
                    "Posts"
                </a>
                <a href="/tags">
                    "Tags"
                </a>
            </nav>
        </header>
    }
}
