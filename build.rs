use pulldown_cmark::{html, CodeBlockKind, CowStr, Event, Options, Parser, Tag, TagEnd};
use std::{env, fs, path::Path};
use syntect::{
    easy::HighlightLines,
    highlighting::ThemeSet,
    html::{
        append_highlighted_html_for_styled_line, start_highlighted_html_snippet, IncludeBackground,
    },
    parsing::SyntaxSet,
    util::LinesWithEndings,
};

static POSTS: &[(&str, &str, &str, &str)] = &[
    // (
    //     "why-i-rewrote-my-blog-in-rust-twice",
    //     "Why I Rewrote My Blog in Rust - Twice",
    //     "2024-10-06",
    //     include_str!("content/posts/why-i-rewrote-my-blog-in-rust-twice.md"),
    // ),
    (
        "alternatives-to-hashicorp-products",
        "Alternatives to HashiCorp products",
        "2024-04-26",
        include_str!("content/posts/alternatives-to-hashicorp-products.md"),
    ),
    (
        "moving-around-efficiently-in-neovim",
        "Moving Around Efficiently in Neovim",
        "2023-06-04",
        include_str!("content/posts/moving-around-efficiently-in-neovim.md"),
    ),
    (
        "arch-linux-installation-guide",
        "Arch Linux Installation Guide",
        "2021-05-22",
        include_str!("content/posts/arch-linux-installation-guide.md"),
    ),
    (
        "convert-from-init-vim-to-init-lua",
        "Convert Neovim config from init.vim to init.lua",
        "2021-03-12",
        include_str!("content/posts/convert-from-init-vim-to-init-lua.md"),
    ),
];

fn main() {
    println!("cargo::rerun-if-changed=content");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("posts_data.rs");

    let entries: Vec<String> = POSTS
        .iter()
        .map(|(id, title, date, markdown)| {
            format!(
                "    (\"{}\", \"{}\", \"{}\", r###\"{}\"###),",
                id,
                title,
                date,
                markdown_to_html(markdown)
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
