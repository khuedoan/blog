use crate::page::base;
use maud::{html, Markup};

pub async fn view() -> Markup {
    base(
        "About",
        html! {
            h1 { "About" }
            p {
                "Hey there, I'm Khue! I'm a Site Reliability Engineer. I have a wide
                range of interests in technology that I love to explore and tinker
                with."
            }
            p {
                "When I was in high school, my dad gave me his old laptop and showed me
                Linux. I was instantly hooked and spent countless hours customizing it
                to my liking, fixing many issues, trying different distros, building my
                own themes, and building Linux From Scratch. Those experiences were
                super fun and ended up paving the way for my career."
            }
            p {
                "Aside from work, I enjoy staying active through calisthenics and
                swimming. I'm also an avid gamer and enjoy playing a variety of games
                such as Overwatch, Elden Ring and Hades. I play all my games on Linux,
                thanks to Wine and Proton."
            }
            p {
                "If you're interested in checking out some of the projects I've been
                working on, be sure to take a look "
                a target="_blank" href="https://github.com/khuedoan" { "here" }
                ". Who knows, you might find something that scratches your itch!"
            }
        },
    )
}
