use maud::{html, Markup, DOCTYPE};

fn head(title: &str) -> Markup {
    html! {
        head {
            meta charset="utf-8";
            meta name="description" content="Khue's personal website";
            meta name="viewport" content="width=device-width, initial-scale=1";
            meta name="color-scheme" content="light dark";
            link rel="stylesheet" href="/pico.min.css";
            script src="/htmx.min.js" defer {}
            title { (title) }
        }
    }
}

pub fn base(title: &str, body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            (head(title))
            body {
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
    }
}
