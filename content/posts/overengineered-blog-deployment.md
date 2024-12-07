In this post, I’m not going into the “why”, as my only reason is “just
because”. I’ll simply describe my not-so-simple setup.

Before I start, here's a meme I found on Twitter (apparently it's called X
now):

![https://twitter.com/dexhorthy/status/856639005462417409/photo/1](https://pbs.twimg.com/media/C-NknkeUwAAxSQs?format=jpg&name=large)

## Project structure

My [blog](https://github.com/khuedoan/blog) project looks like this (truncated for brevity):

```sh
.
├── build.rs
├── Cargo.toml
├── content
│   └── posts
│       ├── post-1.md
│       ├── post-2.md
│       └── post-3.md
├── Dockerfile
├── .envrc
├── flake.nix
├── Makefile
├── README.md
├── src
└── tests
```

Posts are written in Markdown and placed in `content/posts`. As you may
noticed, the blog is built with Rust, but I'll leave that for another post.

## Writing a new post

Let's say I want to write a new post, `post-4`.

First, I'll create a new Markdown file:

```sh
└── content
    └── posts
        ├── post-1.md
        ├── post-2.md
        ├── post-3.md
        └── post-4.md
```

After finishing the content and it's time to publish, I'll include it in the
`build.rs` list of posts that I want to publish:

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

That's fairly straightforward.

## Testing

Because this is an actual program, I have some unit tests in the projects. I
can run them with:

```sh
make test
```

## Git push to deploy

Once I’m satisfied with that, I can deploy the site. I have a custom
Heroku-like "PaaS" setup that allows me to deploy with a simple git push:

```sh
git push production
```

This will run the optional CI script, which is essentially just a git
`pre-receive` hook that runs this `ci` target in the `Makefile` with caching.

The CI output is printed directly to my terminal, so I don't need a to go to a
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

Then my build system see that the project has a `Dockerfile`, so it build and
push to my registry, again with caching and output printed to my terminal:

```sh
# Truncated for brevity
remote: #15 exporting to image
remote: #15 exporting layers 0.0s done
remote: #15 writing image sha256:57e2a535588a54cea961c942e5b3efcd1ef16855b5c9865ca57a03c6203e5e33 done
remote: #15 naming to docker.io/library/blog:db853a79c495a9f8b88cf3425766e8aea418a5cd done
remote: #15 DONE 0.1s
remote: docker.io/khuedoan/blog:db853a79c495a9f8b88cf3425766e8aea418a5cd
```

Once the image is pushed, the deployment script on the server will update my
GitOps repository with the new image tag and commit it:

```sh
# Truncated for brevity
remote: [master 23b3caa] chore(blog): update image tag to db853a79c495a9f8b88cf3425766e8aea418a5cd
remote:  1 file changed, 1 insertion(+), 1 deletion(-)
remote: /var/lib/micropaas/repos/horus.git /tmp/tmp.S4sW4fha6R /var/lib/micropaas/repos/blog.git
```

```diff
+++ b/apps/blog/values.yaml
@@ -7,7 +7,7 @@ app-template:
         main:
           image:
             repository: docker.io/khuedoan/blog
-            tag: b91c794c516c3a6f3aa1268175ac820dd9b61e88
+            tag: db853a79c495a9f8b88cf3425766e8aea418a5cd
```

Oh did I mentioned that I use Kubernetes as the deployment platform? I have a
few clusters running in my labs, and the blog is deployed to one of them.

The above file is a [Helm](https://helm.sh) values file that is used to deploy
the blog, which contains some Kubernetes resources like `Deployment`,
`Service`, and `Ingress`. Usually when I push new content, I just need to bump
the image tag in the values file, so that's the part that I automated in my
custom "PaaS".

This will then trigger the GitOps controller in my Kubernetes cluster
(currently [ArgoCD](https://argo-cd.readthedocs.io/)), which pull the change
from the GitOps repo, and apply it to the cluster.

Once applied, Kubernetes will do it thing and rolling update the pods with zero
downtime!
