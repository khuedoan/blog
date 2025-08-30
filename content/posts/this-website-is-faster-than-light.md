# This website is faster than light

Click around this site for a bit - you'll notice something unusual.

It feels _instant_. Pages appear faster than you can even lift your mouse
button. Even if you're halfway across the world from my servers, links still
open like there's no distance at all. Faster than the speed of light!

Of course, nothing is actually breaking physics. But the experience sure makes
it feel like it. At first glance this site looks like a plain static site. It
isn't. There are a few tricks under the hood that make it this snappy.

Let's peel back the layers.

## "Act on press"

This idea came from a tweet by the legendary programmer [John
Carmack](https://en.wikipedia.org/wiki/John_Carmack):

> Act on press
>
> This is a UI design hill I will die on, and it dismays me how often and hard
> I have had to fight for it.
>
> Almost all interaction methods have a “press” and “release” event associated
> with them. Whenever possible, you should “do the thing” when you get the press
> event instead of waiting for the release event, because it makes the
> interaction feel substantially more responsive, and it reduces user errors by
> not allowing the focus to slide out of the hot box between press and release.
>
> ... (see the [full tweet](https://x.com/ID_AA_Carmack/status/1787850053912064005))

As someone who plays a lot of fast-paced FPS games, I'm hypersensitive to
response time. When browsing the web, it usually takes me about ~100-150ms just
to release the mouse button. By switching link behavior from "navigate on mouse
release" to "navigate on mouse down", I instantly save 100-150ms of perceived
latency. For context, a half-globe round trip is ~300ms, so this little change
basically cuts latency in half.

To do this, I built a `<FastA/>` component - a tiny wrapper around the normal
`<a/>` tag - that fires on press instead of release (the code is truncated
here, but the full source is linked at the end of this post):

```rs
// This could probably be implemented in a better way,
// shoot me an email if you know a better way :)
#[component]
pub fn FastA<H>(
    // truncated

    view! {
        <a
            href=path.clone()
            target=target
            class=class.map(Oco::from)
            on:mousedown=move |event| {
                if is_left_click(&event) {
                    event.prevent_default();
                    navigate(&path, Default::default());
                }
            }
        >
            {children()}
        </a>
    }
}
```

It can be used as a replacement for `<a/>`, like so:

```diff
-<a href="/contact">"Contact"</a>
+<FastA href="/contact">"Contact"</FastA>
```

The `on:mousedown` event still degrades gracefully - if the user's browser
doesn't support WASM, it just falls back to the normal behavior.

Did I say [WASM](https://webassembly.org)?

## Rust, Leptos, and zero network trips

As you may notice, the code above looks like Rust - and that's because it is.
The site is built with [Leptos](https://leptos.dev), a reactive web framework
for Rust.

I have a `markdown_to_html()` function that reads Markdown content and outputs
HTML, adds syntax highlighting, and so on. Then there are functions to
`list_posts()` and `get_post()`. You know, standard blog stuff.

The twist is that the same function runs on both the backend and the frontend.
When you click a post, the `get_post()` function returns the content. If WASM
is supported, it runs right in the browser. If not, it runs on the server.

But doesn't that still require network round trips to get the post from the
server? Short answer: nope - if your browser supports WASM.

Here's where it gets fun: because the Leptos framework is
[isomorphic](https://book.leptos.dev/server/index.html#working-with-the-server),
I can bake all the post content directly into both the server-side Linux binary
and the client-side WASM binary - while still sharing the exact same logic on
both ends.

For a while, I was just tossing raw Markdown in with
[include_str](https://doc.rust-lang.org/std/macro.include_str.html) and
[include_dir](https://docs.rs/include_dir/latest/include_dir), then parsing it
at runtime using
[pulldown-cmark](https://pulldown-cmark.github.io/pulldown-cmark) and
[syntect](https://docs.rs/syntect). It worked, but it made the binaries chunky
and wasted CPU cycles. Now I render everything at compile time using the same
libraries - basically like a static site generator.

To pull that off, I use a `build.rs` script that runs at compile time. It reads
all the Markdown files, converts them to HTML, adds syntax highlighting, and
then bakes everything into a single static array.

```rs
fn main() {
    let dest_path = Path::new(&out_dir).join("posts_data.rs");

    let entries: Vec<String> = POSTS
        .map(|(date, id)| {
            let markdown = fs::read_to_string(format!("./content/posts/{id}.md"))
            // truncated
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
}
```

This spits out a `posts_data.rs` file with a static `POSTS` array, which then
gets baked into the binaries at compile time. From there, the application logic
can read the rendered posts with functions like these:

```rs
include!(concat!(env!("OUT_DIR"), "/posts_data.rs"));

pub fn list_posts() -> Vec<PostMetadata> {
    POSTS
        .iter()
        // truncated
        .collect()
}

pub fn get_post(path: String) -> Option<(PostMetadata, String)> {
    POSTS
        .iter()
        .filter_map(|(id, title, date, content)| {
            // truncated
        })
        .next()
}
```

The result:

- With WASM, it's blazingly fast™. You can even go offline after the first load
  and still read posts.
- Without WASM, it falls back to good old server-rendered HTML.
- On the very first load, it uses server-side rendering just like without WASM,
  and then WASM takes over once it's ready - so you don't have to wait for the
  binary to download before seeing content.

There's one catch: since the content is baked right into the binary, the data
transfer can get heavier if I ever decide to publish a mountain of posts. But
let's be honest - I'm way too lazy to write that many. At the time of writing,
the WASM binary is only about 150KB compressed.

## Squeezing the WASM binary

Besides rendering all Markdown at compile time to avoid extra runtime
dependencies, we can shrink the WASM binary even further using some compiler
flags in `Cargo.toml`:

```toml
[profile.wasm-release]
opt-level = 'z'
lto = true
codegen-units = 1
panic = "abort"
```

This is also [covered in the Leptos
book](https://book.leptos.dev/deployment/binary_size.html), but I'll break down
those options a bit more:

- `opt-level = 'z'`: Optimize for the smallest size instead of raw execution
  speed, the execution speed is already blazingly fast™.
- `lto = true`: Link Time Optimization (LTO) runs an additional optimization
  pass across the entire program after all crates are compiled.
- `codegen-units = 1`: By default, Rust splits the code into multiple codegen
  units to compile faster on multiple CPU cores. For release builds, reducing
  this to 1 allows the compiler to see the whole program at once and optimize
  more aggressively, which usually means smaller binaries (though it takes longer
  to compile).
- `panic = "abort"`: Normally, Rust includes a stack unwinding mechanism for
  handling panics, which adds extra code to the binary. By switching to `abort`,
  the program will immediately terminate on panic - so just don't panic ;)

## What's next

While the site is very fast once the WASM binary is loaded, there are still
ways to improve the initial page load - such as adding more nodes in different
regions or tuning CDN caching. But since my self-hosting setup is already a bit
over-engineered, I'll save that for another weekend.

Stay tuned for a future post on how I host my blog on a highly available,
multi-region, multi-cloud hybrid-metal self-hosting platform with an IPv6-only
WireGuard mesh and `$RANDOM_BUZZWORD`! <small><small>(no, I'm not
joking)</small></small>

All source code for the above can be found at my [blog
repo](https://github.com/khuedoan/blog)
([permalink](https://github.com/khuedoan/blog/tree/a6b5a2c7c5b65566777b486fb672513b242abe9c),
[mirror](https://code.khuedoan.com/khuedoan/blog))
