export default function PostBody({ body }: { body: string }) {
  return (
    <article className="prose">
      <div dangerouslySetInnerHTML={{ __html: body }} />
    </article>
  );
}
