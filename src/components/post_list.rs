use blog::PostData;
use leptos::*;
use std::collections::HashMap;

#[component]
fn PostCover(cx: Scope, href: String, src: String) -> impl IntoView {
    view! { cx,
        <a href=href>
            <img
                class="absolute inset-0 h-full w-full rounded-2xl bg-gray-50 object-cover"
                src=src
                // TODO
                alt=""
                width="720"
                height="1280"
            />
            <div class="absolute inset-0 rounded-2xl ring-1 ring-inset ring-gray-900/10"></div>

        </a>
    }
}

#[component]
fn TagLabel(cx: Scope, tag: String) -> impl IntoView {
    view! { cx,
        <a
            href=format!("/tags/{tag}")
            class="relative z-10 rounded-full bg-gray-50 py-1.5 px-3 font-medium text-gray-600 hover:bg-gray-100"
        >
            {tag}
        </a>
    }
}

#[component]
fn PostPreview(cx: Scope, id: String, post: PostData) -> impl IntoView {
    view! { cx,
        <article key=id class="relative isolate flex flex-col gap-8 lg:flex-row">
            <div class="relative aspect-[16/9] lg:w-64 lg:shrink-0">
                <PostCover
                    // TODO do not hard code
                    href=format!("/posts/TODO")
                    //www.khuedoan.com/posts/moving-around-efficiently-in-neovim/images/cover.png"
                    //www.khuedoan.com/posts/moving-around-efficiently-in-neovim/images/cover.png"
                    src="https://www.khuedoan.com/posts/moving-around-efficiently-in-neovim/images/cover.png"
                        .to_string()
                />
            </div>
            <div>
                <div class="flex items-center gap-x-4 text-xs">
                    <p>
                        // TODO parse date
                        {post.metadata.date}
                    </p>
                    {post
                        .metadata
                        .tags
                        .iter()
                        .map(|tag| view! { cx, <TagLabel tag=tag.clone()/> })
                        .collect_view(cx)}
                </div>
                <div class="group relative max-w-xl">
                    <h2 class="mt-3 text-lg font-semibold leading-6 text-gray-900 group-hover:text-gray-600">
                        // TODO do not hard code
                        // <a href={format!("/posts/{post.path}")}>
                        <a href="/">
                            <span class="absolute inset-0"></span>

                            // TODO

                            {post.metadata.title}
                        </a>
                    </h2>
                    <div class="prose mt-5 text-sm leading-6 text-gray-600">
                        // TODO render Markdown
                        <p>{post.metadata.summary}</p>
                    </div>
                </div>
            </div>
        </article>
    }
}

#[component]
pub fn PostList(cx: Scope, posts: HashMap<String, PostData>) -> impl IntoView {
    view! { cx,
        <div class="mx-auto max-w-3xl px-6 lg:px-8">
            <div class="mt-16 space-y-10 lg:mt-10 lg:space-y-10">
                {posts
                    .iter()
                    .map(|(id, post)| {
                        view! { cx, <PostPreview id=id.to_string() post=post.clone()/> }
                    })
                    .collect_view(cx)}
            </div>
        </div>
    }
}
