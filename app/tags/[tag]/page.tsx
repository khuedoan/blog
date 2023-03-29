import { getAllTags, getPostsByTag } from "@/lib/post";
import PostPreview from "@/components/post-preview";

export async function generateStaticParams() {
  return getAllTags().map((tag) => ({
    tag: tag,
  }));
}

export default function Tag({ params }: { params: { tag: string } }) {
  const posts = getPostsByTag(params.tag);

  return (
    <div className="mx-auto max-w-3xl px-6 lg:px-8">
      <h1 className="mt-2 py-20 text-center text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">
        <code>{params.tag}</code>
      </h1>
      <div className="mt-16 space-y-10 lg:mt-10 lg:space-y-10">
        {posts.map((post) => {
          return (
            <PostPreview
              key={post.slug}
              title={post.title}
              slug={post.slug}
              date={post.date}
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
