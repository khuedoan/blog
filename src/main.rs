use leptos::*;
use leptos_router::*;

pub mod components;
pub mod pages;

#[component]
fn App(cx: Scope) -> impl IntoView {
    use pages::about::*;
    use pages::home::*;
    use pages::posts::*;
    use pages::tags::*;

    use components::footer::*;
    use components::header::*;

    view! { cx,
        <Router>
            <Header />
            <main>
                <Routes>
                    <Route path="/" view=|cx| view! { cx, <Home /> }/>
                    <Route path="/about" view=|cx| view! { cx, <About /> }/>
                    <Route path="/posts" view=|cx| view! { cx, <Posts /> }/>
                    <Route path="/posts/:id" view=|cx| view! { cx, <Post /> }/>
                    <Route path="/tags" view=|cx| view! { cx, <Tags /> }/>
                </Routes>
            </main>
            <Footer />
        </Router>
    }
}

fn main() {
    mount_to_body(|cx| {
        view! { cx,
            <App />
        }
    })
}
