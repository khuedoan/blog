use leptos::*;

#[component]
pub fn PostContent(cx: Scope, content: String) -> impl IntoView {
    let parser = pulldown_cmark::Parser::new(&content);
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);

    view! { cx,
        <div inner_html=html/>
    }
}
