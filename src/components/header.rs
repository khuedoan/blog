use leptos::*;

#[component]
pub fn Header(cx: Scope) -> impl IntoView {
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
                        // TODO remove external hosting
                        src="https://avatars.githubusercontent.com/u/27996771?v=4"
                        alt="avatar"
                        width="64"
                        height="64"
                        href="/"
                    />
                </a>
                <div class="flex gap-x-6 lg:gap-x-12">
                    <a class="text-sm font-semibold leading-6 text-gray-900" href="/about">
                        "About"
                    </a>
                    <a class="text-sm font-semibold leading-6 text-gray-900" href="/posts">
                        "Posts"
                    </a>
                    <a class="text-sm font-semibold leading-6 text-gray-900" href="/tags">
                        "Tags"
                    </a>
                </div>
            </nav>
        </header>
    }
}
