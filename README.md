# [khuedoan.com](https://khuedoan.com) ![status](https://img.shields.io/website?label=status&style=flat-square&url=https%3A%2F%2Fkhuedoan.com)

This is the source code of my personal website at [khuedoan.com](https://khuedoan.com), built with:

- [Rust](https://www.rust-lang.org)
- [Axum](https://github.com/tokio-rs/axum)
- [Askama](https://github.com/djc/askama)
- [Pico CSS](https://picocss.com)
- [HTMX](https://htmx.org)

You can check out older branches for previous implementations:

- [Rust/Axum/Leptos/WASM/Tailwind CSS)](https://github.com/khuedoan/blog/tree/leptos) (2023)
- [NextJS/Tailwind CSS](https://github.com/khuedoan/blog/tree/nextjs) (2023)
- [Hugo/PaperMod](https://github.com/khuedoan/blog/tree/hugo) (2021)
- Plain HTML (2020)

## Project structure

- `content/`: content written in Markdown
- `src/`: pages and components, inspired by [Next.js](http://nextjs.org)
- `public/`: static assets

## Content convention

```
content
└── posts
    └── example-post.md
public
├── any-static-file-here.txt
└── images
    ├── avatar.jpg
    ├── cat-picture.jpg
    └── example-post-cover.png
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

- Docker image: `Dockerfile`
- Pipelines: `.woodpecker/`
- Kubernetes configuration: see <https://github.com/khuedoan/homelab/tree/master/apps/blog>

## Acknowledgements

- [Leptos documentation](https://leptos-rs.github.io/leptos): code examples
- [leptos-rs/start-axum](https://github.com/leptos-rs/start-axum): code examples
- [leptos-rs/leptos-website](https://github.com/leptos-rs/leptos-website): code examples, Nix and Docker config
- [Tailwind UI](https://tailwindui.com): styled components (highly recommended, but it is not free; you can get lifetime access for a one-time fee)
- [Making a dev shell with nix flakes](https://fasterthanli.me/series/building-a-rust-service-with-nix/part-10): Nix flake
- [Comment by ScottAbbey in pulldown-cmark#167](https://github.com/pulldown-cmark/pulldown-cmark/issues/167#issuecomment-448491422) and [eguiraud/highlight-pulldown](https://gitlab.com/eguiraud/highlight-pulldown): Syntax highlighting for Markdown using `pulldown-cmark` and `syntect`.
