use leptos::*;

#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    view! { cx,
        <header>
            <nav>
                <a>
                    "Khue Doan"
                </a>
                <a>
                    "About"
                </a>
                <a>
                    "Posts"
                </a>
                <a>
                    "Tags"
                </a>
            </nav>
        </header>
    }
}
