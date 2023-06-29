use leptos::*;
use pulldown_cmark::{html, Options, Parser};

#[component]
pub fn PostContent(cx: Scope, content: String) -> impl IntoView {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
    let parser = Parser::new_ext(&content, options);
    let mut html = String::new();
    html::push_html(&mut html, parser);

    view! { cx,
        <div inner_html=html/>
    }
}
