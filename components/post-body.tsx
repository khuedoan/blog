import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";

export default function PostBody({ content }: { content: string }) {
  return (
    <article className="prose max-w-none">
      {/* eslint-disable-next-line react/no-children-prop */}
      <ReactMarkdown children={content} remarkPlugins={[remarkGfm]} />
    </article>
  );
}
