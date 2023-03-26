import PostBody from "@/components/post-body";
import PostHeader from "@/components/post-header";
import { getPostBySlug, getAllPosts } from "@/lib/post";
import markdownToHtml from "@/lib/markdown";

export async function generateStaticParams() {
  return getAllPosts(["slug"]);
}

export default async function Post({ params }: { params: { slug: string } }) {
  const post = getPostBySlug(params.slug, [
    "content",
    "date",
    "draft",
    "summary",
    "title",
    "tags",
  ]);

  return (
    <>
      <PostHeader title={post.title} date={post.date} />
      <PostBody body={await markdownToHtml(post.content)} />
    </>
  );
}
