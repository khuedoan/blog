import { getAllPosts } from "@/lib/post";
import PostPreview from "./post-preview";

export default function PostList() {
  const posts = getAllPosts(["slug", "title", "date"]);

  return (
    <>
      {posts.map((post) => (
        <PostPreview
          key={post.slug}
          title={post.title}
          slug={post.slug}
          date={post.date.toString()}
        />
      ))}
    </>
  );
}
