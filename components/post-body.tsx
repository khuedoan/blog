import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";

export default function PostBody({ content }: { content: string }) {
  return (
    <article className="prose max-w-none">
      <ReactMarkdown children={content} remarkPlugins={[remarkGfm]} />
    </article>
  );
}
