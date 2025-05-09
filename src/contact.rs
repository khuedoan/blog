use crate::page::base;
use maud::{html, Markup};

pub async fn view() -> Markup {
    base(
        "Contact",
        html! {
            h1 { "Contact" }
            pre { r#"
┌─────email─────┐
│               │
│    ┌──website─┐
│    │          │
mail@khuedoan.com
     │      │
     └──────┘
github & linkedin"#
            }
            p { "You can reach me via one of the following links:" }
            ul {
                li { a target="_blank" href="mailto:mail@khuedoan.com" { "Email" } }
                li {
                    a target="_blank" href="https://github.com/khuedoan" { "GitHub" }
                    " (or my Git server at " a target="_blank" href="https://code.khuedoan.com" { "code.khuedoan.com" } ")"
                }
                li { a target="_blank" href="https://linkedin.com/in/khuedoan" { "LinkedIn" } }
                li { a target="_blank" href="https://twitter.com/KhueDoanID" { "Twitter" } }
            }
            p {
                "If you need to send sensitive information, you can encrypt the message
                using my "
                a target="_blank" href="/gpg" { "GPG public key" }
                "."
            }
        },
    )
}
