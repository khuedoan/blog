import PostBody from "@/components/post-body";
import PostHeader from "@/components/post-header";
import { getPostBySlug, getAllPosts } from "@/lib/post";
import markdownToHtml from "@/lib/markdown";

export async function generateStaticParams() {
  return getAllPosts();
}

export default async function Post({ params }: { params: { slug: string } }) {
  const post = getPostBySlug(params.slug);

  return (
    <div className="mx-auto max-w-3xl px-6 lg:px-8">
      <PostHeader title={post.title} date={post.date} />
      <PostBody body={await markdownToHtml(post.content)} />
    </div>
  );
}
