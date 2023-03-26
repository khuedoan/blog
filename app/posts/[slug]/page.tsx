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
    <>
      <PostHeader title={post.title} date={post.date} />
      <PostBody body={await markdownToHtml(post.content)} />
    </>
  );
}
