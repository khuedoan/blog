# [khuedoan.com](https://khuedoan.com) ![status](https://img.shields.io/website?label=status&style=flat-square&url=https%3A%2F%2Fkhuedoan.com)

The source code of my personal website, written in [Rust](https://www.rust-lang.org) using the [Leptos](https://leptos.dev) framework,
and styled with [Tailwind CSS](https://tailwindcss.com).

## Project structure

- `content/`: content written in Markdown
- `src/`: pages and components, inspired by [Next.js](http://nextjs.org)
- `public/`: static assets

## Development

Open a shell that provides the build environment (requires Nix):

```
nix develop
```

Run the development server:

```
make dev
```

Then open <http://localhost:8080>.

## Deployment

TBD
