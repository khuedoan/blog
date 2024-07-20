use maud::{html, Markup};

pub async fn index() -> Markup {
    html! {
        h1 { "Hello, World!" }
    }
}
