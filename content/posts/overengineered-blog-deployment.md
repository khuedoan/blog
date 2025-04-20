# Overengineered blog deployment

Before I begin, here's [a meme I found on Twitter](https://twitter.com/dexhorthy/status/856639005462417409/photo/1):

![https://twitter.com/dexhorthy/status/856639005462417409/photo/1](https://pbs.twimg.com/media/C-NknkeUwAAxSQs?format=jpg&name=large)

In this post, I'm not going into the "why", as my only reason is "just
because". I'll simply describe my not-so-simple setup.

Here’s a companion demo video showcasing the deployment process:

<div style="position:relative; padding-bottom:56.25%; height:0; overflow:hidden; max-width:100%;">
    <iframe
        src="https://www.youtube-nocookie.com/embed/RvB2F-x8fno?si=d7XxSJd7vdhyzf-l"
        title="YouTube video player"
        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
        referrerpolicy="strict-origin-when-cross-origin"
        allowfullscreen
        style="position:absolute; top:0; left:0; width:100%; height:100%;"
    ></iframe>
</div>

## Project structure

The [source code of my blog](https://github.com/khuedoan/blog) looks like this
(truncated for brevity):

```sh
.
├── build.rs
├── Cargo.toml
├── content
│   └── posts
│       ├── post-1.md
│       ├── post-2.md
│       └── post-3.md
├── flake.nix
├── Makefile
├── src
└── tests
```

Posts are written in Markdown and placed in `content/posts`. As you might have
noticed, the blog is written in Rust, but I’ll leave that for another post.

## Writing a new post

Let's say I want to write a new post, `post-4`.

First, I'll create a new Markdown file:

```diff
 └── content
     └── posts
         ├── post-1.md
         ├── post-2.md
         ├── post-3.md
+        └── post-4.md
```

After finishing the content and getting ready to publish, I'll add it to the
`build.rs` list of posts:

```rs
static POSTS: &[(&str, &str, &str, &str)] = &[
    (
        "post-4",
        "Post 4 title",
        "2024-12-08",
        include_str!("./content/posts/post-4.md"),
    ),
    // Truncated for brevity
    // ...
];
```

I can preview the site locally by running:

```sh
make dev
# http://localhost:3000
```

That’s fairly straightforward.

## Testing

Since this is an actual program, I have some unit tests in the project. I can
run them with:

```sh
make test
```

## Git push to deploy

Once I’m satisfied with that, I can deploy the site. I have a custom
Heroku-like "PaaS" setup that allows me to deploy with a simple git push:

```sh
git push production
```

This will run an optional CI script, which is essentially just a git
`pre-receive` hook that runs the `ci` target in the `Makefile`, with caching.

The CI logs is printed directly to my terminal, so I don't need to open a
separate web UI to see the result. It looks something like this:

```sh
# Truncated for brevity
remote: running 6 tests
remote: test public::tests::file_nested_image ... ok
remote: test public::tests::file_htmx ... ok
remote: test posts::tests::rendered_content ... ok
remote: test public::tests::file_gpg ... ok
remote: test public::tests::file_not_found ... ok
remote: test public::tests::file_picocss ... ok
```

You might wonder how my custom CI knows which dependencies it needs to run the
tests. All of my dependencies are defined in the `flake.nix` file, and the CI
system just uses that to get all required dependencies, exactly the same
version I have on my local machine (see also [Nix and direnv - a match made in
heaven](https://www.khuedoan.com/posts/nix-and-direnv-a-match-made-in-heaven)).

In my build system, I use [Nixpacks](https://nixpacks.com) to automatically
detect and build my project into a container image without requiring a
`Dockerfile` (though I can fall back to a `Dockerfile` for projects that cannot
be detected by Nixpacks if needed):

```sh
# Truncated for brevity
remote: === Successfully Built! ===
remote: 
remote: Run:
remote:   docker run -it e0bfdddf-e414-4749-ba92-6f68b06a8f9c
remote: docker.io/khuedoan/blog:62872ac3a8fdac2b5dfbafea0421b436ab5bffdb
```

> Disclaimer: I worked at the company that created Nixpacks, but I’m sharing it
> because I genuinely think it’s a great tool, not because I’m being paid or
> sponsored.

Once the image is pushed, the deployment script on the server updates my GitOps
repository with the new image tag and commits it:

```diff
+++ b/apps/blog/values.yaml
@@ -7,7 +7,7 @@ app-template:
         main:
           image:
             repository: docker.io/khuedoan/blog
-            tag: b91c794c516c3a6f3aa1268175ac820dd9b61e88
+            tag: db853a79c495a9f8b88cf3425766e8aea418a5cd
```

```sh
remote: [master 23b3caa] chore(blog): update image tag to db853a79c495a9f8b88cf3425766e8aea418a5cd
remote:  1 file changed, 1 insertion(+), 1 deletion(-)
remote: /var/lib/micropaas/repos/horus.git /tmp/tmp.S4sW4fha6R /var/lib/micropaas/repos/blog.git
```

Oh, did I mention that I use Kubernetes as the deployment platform? I have a
few clusters running in my labs, and the blog is deployed to one of them.
That’s why I linked the meme at the beginning of the post - my blog is just a
tiny brick on the giant truck :D.

The file above is a [Helm](https://helm.sh) values file used to deploy the
blog, containing Kubernetes resources like `Deployment`, `Service`, `Ingress`,
etc. When I push new content, I usually just need to update the image tag in
the values file, which is the part automated in my custom "PaaS".

It also sends a webhook to trigger the GitOps controller in my Kubernetes
cluster (currently [ArgoCD](https://argo-cd.readthedocs.io/)) to sync, pulling
changes from the GitOps repo and applying them to the cluster. The webhook
isn’t strictly required but speeds up deployment by eliminating the need to
wait for the controller to poll the repo.

Once applied, Kubernetes does its thing:

```diff
 NAMESPACE  NAME                   READY  STATUS       AGE
-blog       blog-5ccc675f94-kvfzn  1/1    Terminating  12m
 blog       blog-5ccc675f94-lgjt6  1/1    Running      12m
+blog       blog-69fcfb78dd-2fv4g  0/1    Pending      0s
 blog       blog-69fcfb78dd-nspdp  1/1    Running      3s
```

My blog deployment has 2 replicas, and in the logs above I can see:

- One new pod is healthy (`blog-69fcfb78dd-nspdp`)
- One old pod is replaced by the new one and is terminating (`blog-5ccc675f94-kvfzn`)
- One new pod is starting up (`blog-69fcfb78dd-2fv4g`)
- One old pod is still running and will be replaced by the new one once it’s
  ready (`blog-5ccc675f94-lgjt6`)

Every time I deploy a new version, Kubernetes gradually replaces the old pods
with new ones, with zero downtime!

## Conclusion

That’s how I deploy my blog. It’s a _bit_ overengineered for a blog, and I must
admit that I spent more time building it than writing content. But hey, it’s
fun!

You can find the code for all of the above at:

- [blog](https://github.com/khuedoan/blog) (this blog)
- [horus](https://github.com/khuedoan/horus) (one of my cluster)
- [homelab](https://github.com/khuedoan/homelab) (another cluster)
- [micropaas](https://github.com/khuedoan/micropaas) (my current build and deploy
  scripts, but I'm rewriting it into a more polished product)
- A few more projects I haven’t published yet, so stay tuned!
