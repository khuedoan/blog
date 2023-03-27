import ReactMarkdown from "react-markdown";
export default function PostBody({ content }: { content: string }) {
  return (
    <article className="prose max-w-none">
      <ReactMarkdown children={content} />
    </article>
  );
}
