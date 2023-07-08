use leptos::*;

#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    let nav_items = [("About", "/about"), ("Posts", "/posts"), ("Tags", "/tags")];

    view! { cx,
        <header class="relative z-50">
            <nav
                class="mx-auto flex max-w-7xl items-center justify-between p-6 lg:px-8"
                aria-label="Global"
            >
                <a href="/" class="-m-1.5 p-1.5">
                    <span class="sr-only">"Khue Doan"</span>
                    <img
                        class="h-8 w-auto rounded-full"
                        src="/images/avatar.jpg"
                        alt="avatar"
                        width="64"
                        height="64"
                        href="/"
                    />
                </a>
                <div class="flex gap-x-6 lg:gap-x-12">
                    {nav_items
                        .iter()
                        .map(|(name, href)| {
                            view! { cx,
                                <a
                                    class="text-sm font-semibold leading-6 text-gray-900"
                                    href=href.to_string()
                                >
                                    {name.to_string()}
                                </a>
                            }
                        })
                        .collect_view(cx)}
                </div>
            </nav>
        </header>
    }
}
