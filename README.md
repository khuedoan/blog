# [khuedoan.com](https://khuedoan.com) ![status](https://img.shields.io/website?label=status&style=flat-square&url=https%3A%2F%2Fkhuedoan.com)

This is the source code of my personal website at <https://khuedoan.com>, written in [Rust](https://www.rust-lang.org)
using the [Leptos](https://leptos.dev) framework, and styled with [Tailwind CSS](https://tailwindcss.com).

## Project structure

- `content/`: content written in Markdown
- `src/`: pages and components, inspired by [Next.js](http://nextjs.org)
- `public/`: static assets

## Content convention

```
content
├── about.md
└── posts
    └── example-post.md
public
├── any-static-file-here.txt
└── images
    ├── avatar.jpg
    ├── cat-picture.jpg
    └── example-post-cover.png
```

Posts are placed in `content/posts`, each post is a Markdown file with a YAML front matter block:

```
---
title: Example post
summary: This is an example post
date: 2023-01-01T00:00:00+07:00
cover: /images/example-post-cover.png
tags:
  - foo
  - bar
---

## Example heading

![Files inside `./public` can be referenced by your code starting from the base URL](/images/cat-picture.jpg)

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
- Build pipeline: `.ci/master.yaml`
- Kubernetes manifests: see <https://github.com/khuedoan/homelab/tree/master/apps/blog>

## Acknowledgements

- [Leptos documentation](https://leptos-rs.github.io/leptos): code examples
- [leptos-rs/start-axum](https://github.com/leptos-rs/start-axum): code examples
- [leptos-rs/leptos-website](https://github.com/leptos-rs/leptos-website): code examples, Nix and Docker config
- [Tailwind UI](https://tailwindui.com): styled components (highly recommended, but it is not free; you can get lifetime access for a one-time fee)
