use maud::{html, Markup, DOCTYPE};

fn header(title: &str) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        meta name="viewport" content="width=device-width, initial-scale=1";
        meta name="color-scheme" content="light dark";
        link rel="stylesheet" href="/pico.min.css";
        script src="/htmx.min.js" {}
        title { (title) }
    }
}

pub fn base(title: &str, body: Markup) -> Markup {
    html! {
        (header(title))
        header class="container" {
            nav {
                ul {
                    li { strong { a href="/" { "Khue Doan" } } }
                }
                ul {
                    li { a href="/contact" { "Contact" } }
                    li { a href="/about" { "About" } }
                }
            }
        }
        main class="container" {
            h1 { (title) }
            (body)
        }
    }
}
