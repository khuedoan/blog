use crate::page::base;
use maud::{html, Markup};

pub async fn view() -> Markup {
    base(
        "Contact",
        html! {
            p { "You can reach me via one of the following links:" }
            ul {
                li { a target="_blank" href="mailto:mail@khuedoan.com" { "Email" } }
                li { a target="_blank" href="https://github.com/khuedoan" { "GitHub" } }
                li { a target="_blank" href="https://linkedin.com/in/khuedoan" { "LinkedIn" } }
                li { a target="_blank" href="https://twitter.com/KhueDoanID" { "Twitter" } }
            }
            p {
                "If you need to send sensitive information, you can encrypt the message
                using my"
            }
            a target="_blank" href="/gpg" { "GPG public key" }
        },
    )
}
