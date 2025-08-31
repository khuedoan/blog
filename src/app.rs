use crate::posts::{get_post, list_posts};
use leptos::{ev::MouseEvent, prelude::*};
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes, ToHref},
    hooks::{use_navigate, use_params_map},
    path,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <meta name="description" content="Khue's personal website" />
                <meta name="color-scheme" content="light dark" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="pico" href="/pico.min.css" />
        <Title text="Khue Doan" />
        <Router>
            <header class="container">
                <Nav />
            </header>
            <main class="container">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=StaticSegment("/about") view=About />
                    <Route path=StaticSegment("/contact") view=Contact />
                    <Route path=path!("/posts/:id") view=PostPage />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Nav() -> impl IntoView {
    view! {
        <nav>
            <ul>
                <li>
                    <strong>
                        <FastA href="/">"Khue Doan"</FastA>
                    </strong>
                </li>
            </ul>
            <ul>
                <li>
                    <FastA href="/contact">"Contact"</FastA>
                </li>
                <li>
                    <FastA href="/about">"About"</FastA>
                </li>
            </ul>
        </nav>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <h1>"Hi there, I’m Khue 👋"</h1>
        <p>
            "Welcome to my website, where I write about Linux, homelab, workflow optimization, and more!"
        </p>
        <PostList />
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <Title text="Khue Doan - About" />
        <h1>About</h1>
        <p>
            "Hey there, I'm Khue! I'm a Site Reliability Engineer. I have a wide
            range of interests in technology that I love to explore and tinker
            with."
        </p>
        <p>
            "When I was in high school, my dad gave me his old laptop and showed me
            Linux. I was instantly hooked and spent countless hours customizing it
            to my liking, fixing many issues, trying different distros, building my
            own themes, and building Linux From Scratch. Those experiences were
            super fun and ended up paving the way for my career."
        </p>
        <p>
            "Aside from work, I enjoy staying active through calisthenics and
            swimming. I'm also an avid gamer and enjoy playing a variety of games
            such as Overwatch, Elden Ring and Hades. I play all my games on Linux,
            thanks to Wine and Proton."
        </p>
        <p>
            "If you're interested in checking out some of the projects I've been
            working on, be sure to take a look "
            <a target="_blank" href="https://github.com/khuedoan">
                here
            </a> ". Who knows, you might find something that scratches your itch!"
        </p>
    }
}

#[component]
fn Contact() -> impl IntoView {
    view! {
        <Title text="Khue Doan - Contact" />
        <h1>Contact</h1>
        <pre>
            r#"
            ┌─────email─────┐
            │               │
            │    ┌──website─┐
            │    │          │
            mail@khuedoan.com
                 │      │
                 └──────┘
            github & linkedin"#
        </pre>
        <p>You can reach me via one of the following links:</p>
        <ul>
            <li>
                <a target="_blank" href="mailto:mail@khuedoan.com">
                    Email
                </a>
            </li>
            <li>
                <a target="_blank" href="https://github.com/khuedoan">
                    GitHub
                </a>
                " (or my Git server at "
                <a target="_blank" href="https://code.khuedoan.com">
                    code.khuedoan.com
                </a>
                ")"
            </li>
            <li>
                <a target="_blank" href="https://linkedin.com/in/khuedoan">
                    LinkedIn
                </a>
            </li>
            <li>
                <a target="_blank" href="https://twitter.com/KhueDoanID">
                    Twitter
                </a>
            </li>
        </ul>
        <p>
            "If you need to send sensitive information, you can encrypt the message
            using my " <a target="_blank" href="/gpg">
                GPG public key
            </a> "."
        </p>
    }
}

#[component]
pub fn FastA<H>(
    href: H,
    #[prop(optional)] target: Option<&'static str>,
    #[prop(optional, into)] class: Option<String>,
    children: Children,
) -> impl IntoView
where
    H: ToHref + Send + Sync + 'static,
{
    let navigate = use_navigate();
    let path = href.to_href()();

    fn is_left_click(event: &MouseEvent) -> bool {
        event.button() == 0
            && !event.meta_key()
            && !event.ctrl_key()
            && !event.shift_key()
            && !event.alt_key()
    }

    view! {
        <a
            href=path.clone()
            target=target
            class=class.map(Oco::from)
            on:mousedown=move |event| {
                if is_left_click(&event) {
                    event.prevent_default();
                    navigate(&path, Default::default());
                }
            }
        >
            {children()}
        </a>
    }
}

#[component]
fn PostList() -> impl IntoView {
    let posts = list_posts();

    view! {
        <table>
            <thead>
                <tr>
                    <th scope="col">Title</th>
                    <th scope="col">Date</th>
                </tr>
            </thead>
            <tbody>
                {posts
                    .into_iter()
                    .map(|post| {
                        let path = format!("/posts/{}", post.id);
                        view! {
                            <tr>
                                <th scope="row">
                                    <FastA href=path.clone() class="contrast">
                                        {post.title}
                                    </FastA>
                                </th>
                                <th scope="row">{post.date}</th>
                            </tr>
                        }
                    })
                    .collect::<Vec<_>>()}
            </tbody>
        </table>
    }
}

#[component]
fn PostPage() -> impl IntoView {
    let params = use_params_map();
    // TODO avoid unwrap and use type safe params
    let id = move || params.read().get("id").unwrap_or_default();
    let (metadata, html) = get_post(id()).unwrap();
    view! {
        <Title text=metadata.title />
        <div inner_html=html />
    }
}
