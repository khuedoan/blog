use pulldown_cmark::{CodeBlockKind, CowStr, Event, Options, Parser, Tag, TagEnd, html};
use std::{env, fs, path::Path};
use syntect::{
    easy::HighlightLines,
    highlighting::ThemeSet,
    html::{
        IncludeBackground, append_highlighted_html_for_styled_line, start_highlighted_html_snippet,
    },
    parsing::SyntaxSet,
    util::LinesWithEndings,
};

static POSTS: &[(&str, &str)] = &[
    // (
    //     "switching-my-self-hosted-git-servers-to-radicle",
    //     "Switching my self-hosted Git servers to Radicle",
    //     "2024-12-08",
    //     include_str!("./content/posts/switching-my-self-hosted-git-servers-to-radicle.md"),
    // ),
    ("2024-12-08", "overengineered-blog-deployment"),
    ("2024-12-07", "nix-and-direnv-a-match-made-in-heaven"),
    ("2024-08-24", "nixos-cured-my-ocd"),
    ("2024-04-26", "alternatives-to-hashicorp-products"),
    (
        "2023-11-01",
        "optimizing-thinkpad-z13-battery-life-on-linux",
    ),
    ("2023-10-22", "minimal-neovim-setup-from-scratch"),
    ("2023-06-04", "moving-around-efficiently-in-neovim"),
    (
        "2023-03-07",
        "fixing-a-weird-bug-on-the-intel-219-lm-ethernet-controller",
    ),
    (
        "2021-06-05",
        "automating-linux-installation-on-bare-metal-with-a-containerized-pxe-server",
    ),
    ("2021-05-22", "arch-linux-installation-guide"),
    ("2021-03-12", "convert-from-init-vim-to-init-lua"),
];

fn main() {
    println!("cargo::rerun-if-changed=content");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("posts_data.rs");

    let entries: Vec<String> = POSTS
        .iter()
        .map(|(date, id)| {
            let markdown = fs::read_to_string(format!("./content/posts/{id}.md"))
                .unwrap_or_else(|_| panic!("Failed to read Markdown file for {id}"));
            let title = markdown
                .lines()
                .next()
                .and_then(|line| line.strip_prefix("# "))
                .unwrap_or_else(|| panic!("Title not found for {id}"))
                .to_string();
            format!(
                "    (\"{}\", \"{}\", \"{}\", r###\"{}\"###),",
                id,
                title,
                date,
                markdown_to_html(&markdown)
            )
        })
        .collect();

    fs::write(
        dest_path,
        format!(
            "static POSTS: &[(&str, &str, &str, &str)] = &[\n{}\n];",
            entries.join("\n")
        ),
    )
    .unwrap();
}

fn markdown_to_html(markdown: &str) -> String {
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS;
    let parser = Parser::new_ext(markdown, options);
    let events = highlight(parser).into_iter();
    let mut html = String::new();
    html::push_html(&mut html, events);

    html
}

fn highlight(parser: Parser) -> Vec<Event> {
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let plain_text = syntax_set.find_syntax_plain_text();
    let mut syntax = plain_text;
    let theme = &ThemeSet::load_defaults().themes["base16-ocean.dark"];
    let mut events = Vec::new();
    let mut to_highlight = String::new();
    let mut in_code_block = false;

    parser.into_iter().for_each(|event| match event {
        Event::Start(Tag::CodeBlock(kind)) => {
            match kind {
                CodeBlockKind::Fenced(lang) => {
                    syntax = syntax_set.find_syntax_by_token(&lang).unwrap_or(plain_text)
                }
                CodeBlockKind::Indented => {}
            }
            in_code_block = true;
        }
        Event::End(TagEnd::CodeBlock) => {
            if in_code_block {
                // Not using highlighted_html_for_string() because we need to inject the <code> tag
                // for PicoCSS to apply the correct margin
                let mut highlighter = HighlightLines::new(syntax, theme);
                let (mut html, background) = start_highlighted_html_snippet(theme);

                html.push_str("<code>");
                for line in LinesWithEndings::from(&to_highlight) {
                    let regions = highlighter.highlight_line(line, &syntax_set).unwrap();
                    append_highlighted_html_for_styled_line(
                        &regions[..],
                        IncludeBackground::IfDifferent(background),
                        &mut html,
                    )
                    .unwrap();
                }
                html.push_str("</code></pre>\n");

                events.push(Event::Html(CowStr::from(html)));
                to_highlight.clear();
                in_code_block = false;
            }
        }
        Event::Text(t) => {
            if in_code_block {
                to_highlight.push_str(&t);
            } else {
                events.push(Event::Text(t))
            }
        }
        e => {
            events.push(e);
        }
    });

    events
}
