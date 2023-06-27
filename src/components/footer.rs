use leptos::*;

#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
    view! { cx,
        <footer>
            <p>
                "© 2023 Khue Doan. All rights reserved." // TODO do not hard code the year
            </p>
        </footer>
    }
}
