use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::about::*;
use crate::pages::home::*;
use crate::pages::posts::*;
use crate::pages::tags::*;

use crate::components::footer::*;
use crate::components::header::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/blog.css"/>
        <Title text="Khue Doan"/>

        <Router>
            <Header/>
            <main>
                <Routes>
                    <Route path="/" view=|cx| view! { cx, <Home/> }/>
                    <Route path="/about" view=|cx| view! { cx, <About/> }/>
                    <Route path="/posts" view=|cx| view! { cx, <Posts/> }/>
                    <Route path="/posts/:id" view=|cx| view! { cx, <Post/> }/>
                    <Route path="/tags" view=|cx| view! { cx, <Tags/> }/>
                    <Route path="/tags/:id" view=|cx| view! { cx, <Tag/> }/>
                </Routes>
            </main>
            <Footer/>
        </Router>
    }
}
