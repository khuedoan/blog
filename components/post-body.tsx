export default function PostBody({ body }: { body: string }) {
  return (
    <article className="prose max-w-none">
      <div dangerouslySetInnerHTML={{ __html: body }} />
    </article>
  );
}
