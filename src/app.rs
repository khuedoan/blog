use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::error_template::{AppError, ErrorTemplate};

use crate::pages::about::*;
use crate::pages::home::*;
use crate::pages::posts::*;
use crate::pages::tags::*;

use crate::components::footer::*;
use crate::components::header::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/blog.css"/>
        <Title text="Khue Doan"/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <Header/>
            <main>
                <Routes>
                    <Route path="/" view=|| view! { <Home/> }/>
                    <Route path="/about" view=|| view! { <About/> }/>
                    <Route path="/posts" view=|| view! { <Posts/> }/>
                    <Route path="/posts/:id" view=|| view! { <Post/> }/>
                    <Route path="/tags" view=|| view! { <Tags/> }/>
                    <Route path="/tags/:id" view=|| view! { <Tag/> }/>
                </Routes>
            </main>
            <Footer/>
        </Router>
    }
}
