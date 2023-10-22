use leptos::*;
use leptos_icons::AiIcon::*;
use leptos_icons::BiIcon::*;
use leptos_icons::*;

struct Social<'a> {
    name: &'a str,
    url: &'a str,
    icon: Icon,
}

#[component]
fn SocialIcons() -> impl IntoView {
    let socials = [
        Social {
            name: "Mail",
            url: "mailto:mail@khuedoan.com",
            icon: Icon::from(AiMailFilled),
        },
        Social {
            name: "GitHub",
            url: "https://github.com/khuedoan",
            icon: Icon::from(AiGithubFilled),
        },
        Social {
            name: "LinkedIn",
            url: "https://linkedin.com/in/khuedoan",
            icon: Icon::from(AiLinkedinFilled),
        },
        Social {
            name: "Twitter",
            url: "https://twitter.com/KhueDoanID",
            icon: Icon::from(AiTwitterOutlined),
        },
        Social {
            name: "Telegram",
            url: "https://t.me/khuedoan",
            icon: Icon::from(BiTelegram),
        },
    ];

    view! {
        <div class="mt-10 flex justify-center space-x-5">
            {socials
                .iter()
                .map(|social| {
                    view! {
                        <a
                            key=social.name
                            href=social.url
                            target="_blank"
                            class="text-gray-400 hover:text-gray-500"
                        >
                            <span class="sr-only">{social.name}</span>
                            <Icon icon=social.icon class="h-6 w-6"/>
                        </a>
                    }
                })
                .collect_view()}
        </div>
    }
}

#[component]
fn Gradient() -> impl IntoView {
    view! {
        <div
            class="absolute inset-x-0 top-1/2 -z-10 flex -translate-y-1/2 transform-gpu justify-center overflow-hidden blur-3xl sm:bottom-0 sm:right-[calc(50%-6rem)] sm:top-auto sm:translate-y-0 sm:transform-gpu sm:justify-end"
            aria-hidden="true"
        >
            <div
                class="aspect-[1108/632] w-[69.25rem] flex-none bg-gradient-to-r from-[#ff80b5] to-[#9089fc] opacity-25"
                style="clip-path: polygon(73.6% 48.6%, 91.7% 88.5%, 100% 53.9%, 97.4% 18.1%, 92.5% 15.4%, 75.7% 36.3%, 55.3% 52.8%, 46.5% 50.9%, 45% 37.4%, 50.3% 13.1%, 21.3% 36.2%, 0.1% 0.1%, 5.4% 49.1%, 21.4% 36.4%, 58.9% 100%, 73.6% 48.6%)"
            ></div>
        </div>
        <div
            class="absolute left-1/2 right-0 top-full -z-10 hidden -translate-y-1/2 transform-gpu overflow-hidden blur-3xl sm:block"
            aria-hidden="true"
        >
            <div
                class="aspect-[1155/678] w-[72.1875rem] bg-gradient-to-tr from-[#ff80b5] to-[#9089fc] opacity-30"
                style="clip-path: polygon(74.1% 44.1%, 100% 61.6%, 97.5% 26.9%, 85.5% 0.1%, 80.7% 2%, 72.5% 32.5%, 60.2% 62.4%, 52.4% 68.1%, 47.5% 58.3%, 45.2% 34.5%, 27.5% 76.7%, 0.1% 64.9%, 17.9% 100%, 27.6% 76.8%, 76.1% 97.7%, 74.1% 44.1%)"
            ></div>
        </div>
    }
}

#[component]
pub fn Intro() -> impl IntoView {
    view! {
        <div class="relative isolate px-6 pt-14 lg:px-8">
            <Gradient/>
            <div class="mx-auto max-w-2xl py-16 sm:py-24 lg:py-28">
                <div class="text-center">
                    <h1 class="text-4xl font-bold tracking-tight text-gray-900 sm:text-6xl">
                        "Hi there, I’m Khue 👋"
                    </h1>
                    <p class="mt-6 text-lg leading-8 text-gray-600">
                        "Welcome to my website, where I write about Linux, DevOps, homelab, workflow optimization, and more!"
                    </p>
                    <SocialIcons/>
                </div>
            </div>
        </div>
    }
}
