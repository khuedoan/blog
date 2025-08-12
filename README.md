# [khuedoan.com](https://khuedoan.com) ![status](https://img.shields.io/website?label=status&style=flat-square&url=https%3A%2F%2Fkhuedoan.com)

This is the source code of my personal website at [khuedoan.com](https://khuedoan.com), built with:

- [Rust](https://www.rust-lang.org)
- [Axum](https://github.com/tokio-rs/axum)
- [Leptos](https://leptos.dev)
- [Pico CSS](https://picocss.com)

You can check out older branches for previous implementations:

- [Rust/Axum/HTMX/Pico CSS](https://github.com/khuedoan/blog/tree/htmx)
- [Next.js/Tailwind CSS](https://github.com/khuedoan/blog/tree/nextjs)
- [Hugo/PaperMod](https://github.com/khuedoan/blog/tree/hugo)
- Plain HTML

## Project structure

- `content/`: content written in Markdown
- `src/`: pages and components
- `public/`: static assets
- `build.rs`: build logic for rendering Markdown to HTML

## Content convention

```
content
└── posts
    └── example-post.md
public
├── any-static-file-here.txt
└── images
    ├── avatar.jpg
    └── cat-picture.jpg
```

Posts are placed in `content/posts`, each post is a Markdown file. Here's an example post:

```
## Example heading

![Files inside `./public` can be referenced by your code starting from the base URL](/images/cat-picture.webp)

Lorem ipsum dolor sit amet, consectetur adipiscing elit,
sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
```

## Development

Open a shell that provides the build environment (requires Nix):

```
nix develop
```

Run the development server:

```
make dev
```

Then open <http://localhost:3000>.

## Deployment

- Kubernetes configuration:
    - <https://github.com/khuedoan/cloudlab/tree/master/apps/khuedoan/blog>
    - <https://github.com/khuedoan/homelab/tree/master/apps/blog>

## Acknowledgements

- [Axum examples](https://github.com/tokio-rs/axum/blob/main/examples): code examples
- [Leptos book](https://book.leptos.dev): code examples
- [leptos-rs/start-axum](https://github.com/leptos-rs/start-axum): code examples
- [Making a dev shell with nix flakes](https://fasterthanli.me/series/building-a-rust-service-with-nix/part-10): Nix flake
- [Comment by ScottAbbey in pulldown-cmark#167](https://github.com/pulldown-cmark/pulldown-cmark/issues/167#issuecomment-448491422) and [eguiraud/highlight-pulldown](https://gitlab.com/eguiraud/highlight-pulldown): Syntax highlighting for Markdown using `pulldown-cmark` and `syntect`
