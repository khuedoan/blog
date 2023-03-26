import fs from "fs";
import { join } from "path";
import matter from "gray-matter";
import Post from "@/interfaces/post";

const postsDirectory = join(process.cwd(), "content/posts");

export function getPostSlugs() {
  return fs.readdirSync(postsDirectory);
}

export function getPostBySlug(slug: string) {
  const fullPath = join(postsDirectory, `${slug}/index.md`);
  const fileContents = fs.readFileSync(fullPath, "utf8");
  const { data, content } = matter(fileContents);

  const post: Post = {
    content: content ?? "",
    coverImage: data.coverImage ?? "",
    date: data.date ?? "",
    draft: data.draft ?? false,
    slug: slug,
    summary: data.summary ?? "",
    tags: data.tags ?? [],
    title: data.title ?? "",
  };

  return post;
}

export function getAllPosts(): Post[] {
  const slugs = getPostSlugs();
  const posts = slugs
    .map((slug) => getPostBySlug(slug))
    .filter((post) => !post.draft) // TODO auto show draft posts in dev mode
    // sort posts by date in descending order
    .sort((post1, post2) => (post1.date > post2.date ? -1 : 1));

  return posts;
}
