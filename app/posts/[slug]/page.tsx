import PostBody from "@/components/post-body";
import { getPostBySlug, getAllPosts } from "@/lib/post";
import markdownToHtml from "@/lib/markdown";

async function getPost(slug: string) {
  const post = getPostBySlug(slug, [
    "content",
    "date",
    "draft",
    "summary",
    "title",
    "tags",
  ]);

  const content = await markdownToHtml(post.content);
  return {
    ...post,
    content,
  };
}

export async function generateStaticParams() {
  return getAllPosts(["slug"])
}

export default async function Post({ params }: { params: { slug: string } }) {
  return (
    <>
      <PostBody body={(await getPost(params.slug)).content} />
    </>
  );
}
