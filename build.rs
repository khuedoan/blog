use pulldown_cmark::{html, Options, Parser};
use std::{env, fs, path::Path};

static POSTS: &[(&str, &str, &str, &str)] = &[
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
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("posts_data.rs");

    let entries: Vec<String> = POSTS
        .iter()
        .map(|(id, title, date, markdown)| {
            format!(
                "    (\"{}\", \"{}\", \"{}\", r#\"{}\"#),",
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
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, options);
    let mut html = String::new();
    html::push_html(&mut html, parser);

    html
}
