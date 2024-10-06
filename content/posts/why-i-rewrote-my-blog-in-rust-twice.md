I’ve over-engineered my blog to make it blazingly fast, but now I’m scaling
down the complexity while keeping most of the performance gains.

My blog has gone through four major versions:

1. Handwritten HTML on GitHub Pages.
2. Hugo static site with the PaperMod theme, also on GitHub Pages.
3. NextJS with Tailwind CSS, which never published.
4. First Rust rewrite with Axum, Leptos, Tailwind CSS, on Kubernetes.
5. Second Rust rewrite with Axum, Askama, Pico CSS, HTMX, on Kubernetes.

## The stepping stone

While the third version written in NextJS was never published, I still consider
it important because:

- The styling of my blog began with this version, using Tailwind UI by copying
  and pasting React components and styles.
- It showed me how productive TypeScript can be with NextJS.
- Despite that, it made me realize how much I dislike the JavaScript ecosystem.
  I started with the page router, then quickly followed the app router once it
  hit the NextJS 13 stable release, but everything still felt unfinished and
  unstable. There’s constant code churn when upgrading dependencies - and trust
  me, you'll be doing that a lot.

The last point is the main reason I didn’t publish it. I have a bit of "OCD",
and it made me constantly feel like something was dirty and "broken "
underneath. Combined with the fact that `node_modules` quickly becomes the
largest black hole in the universe, I kept pushing it back and stuck with the
tried-and-true static Hugo version.

## Rust to the rescue

### The future is (almost) here

> [Are we WASM yet?](https://www.arewewebyet.org)
>
> Yes! And it's freaking fast!

## The magic of simplicity

Or the lack of it - everything is clear and straightforward, there’s no magic involved.

I can do things like this directly in my Markdown files, with just standard HTML:

```markdown
**This is just Markdown**!

Click this button to show my GPG key:

<button hx-get="/gpg" hx-target="#gpg">Click me</button>
<pre>
    <div id="gpg">The public key will be shown here.</div>
</pre>
```

And here's the result:

> **This is just Markdown**!
>
> Click this button to show my GPG key:
>
> <button hx-get="/gpg" hx-target="#gpg">Click me</button>
> <pre>
>     <div id="gpg">The public key will be shown here.</div>
> </pre>

If you're coming from a React background, you might recognize this as similar
to [MDX](https://mdxjs.com), which lets you use JSX in your Markdown content.
But with this approach, there’s no need for a custom `.mdx` file format - it's
just normal HTML in my `.md` files, with some HTMX actions sprinkled in.
