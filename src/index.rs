use crate::page::base;
use crate::posts::list_posts;
use maud::{html, Markup};

pub async fn view() -> Markup {
    let posts = list_posts();

    base(
        "Hi there, I’m Khue 👋",
        html! {
            p {
                "Welcome to my website, where I write about Linux, DevOps, homelab, workflow optimization, and more!"
            }
            table {
                thead {
                    tr {
                        th scope="col" { "Title" }
                        th scope="col" { "Date" }
                    }
                }
                @for post in posts {
                    tr {
                        th scope="row" {
                            a href=(format!("/posts/{}", post.id)) class="contrast" {
                                (post.title)
                            }
                        }
                        th scope="row" {
                            (post.date)
                        }
                    }
                }
            }
        },
    )
}
