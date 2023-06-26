use leptos::*;
pub mod components;
use components::header::Header;
use components::footer::Footer;
use components::intro::Intro;
use components::post_list::PostList;

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Header />
        <Intro />
        <PostList />
        <Footer />
    }
}

fn main() {
    mount_to_body(|cx| view! { cx,
        <App />
    })
}
