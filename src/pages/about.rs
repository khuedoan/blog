use leptos::*;

#[component]
pub fn About(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="mx-auto max-w-3xl px-6 lg:px-8">
            <h1 class="mt-2 py-20 text-center text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">
                "About"
            </h1>
            <article class="prose max-w-none">
                <p>
                    "Hey there, I'm Khue! I'm a Site Reliability Engineer.
                    I have a wide range of interests in technology that I love to explore
                    and tinker with."
                </p>
                <p>
                    "When I was in high school, my dad gave me his old laptop and showed
                    me Linux. I was instantly hooked and spent countless hours
                    customizing it to my liking, fixing many issues, trying different
                    distros, building my own themes, and building Linux From Scratch.
                    Those experiences were super fun and ended up paving the way for my
                    career."
                </p>
                <p>
                    "Aside from work, I enjoy staying active through calisthenics and
                    swimming. I'm also an avid gamer and enjoy playing a variety of
                    games such as Overwatch, Elden Ring and Hades. I play all my games
                    on Linux, thanks to Wine and Proton."
                </p>
                <p>
                    "If you're interested in checking out some of the projects
                    I've been working on, be sure to take a look at my GitHub
                    find something that scratches your itch!" // TODO add link here
                </p>
            </article>
        </div>
    }
}
