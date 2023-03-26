import { getAllPosts } from "@/lib/post";
import PostPreview from "./post-preview";

export default function PostList() {
  const posts = getAllPosts();

  return (
    <div className="mx-auto max-w-3xl px-6 lg:px-8">
      <div className="mt-16 space-y-10 lg:mt-10 lg:space-y-10">
        {posts.map((post) => {
          return (
            <PostPreview
              key={post.slug}
              title={post.title}
              slug={post.slug}
              date={post.date.toLocaleString()}
              summary={post.summary}
              coverImage={post.coverImage}
              tags={post.tags}
            />
          );
        })}
      </div>
    </div>
  );
}
