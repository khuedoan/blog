export default function PostLayout({
  children,
}: {
  children: React.ReactNode,
}) {
  return (
    <div className="bg-white py-32 px-6 lg:px-8">
      <div className="mx-auto max-w-3xl text-base leading-7 text-gray-700">
        <article className="prose">
          {children}
        </article>
      </div>
    </div>
  );
}
