import Link from "next/link";

export default function PostPreview({
  title,
  date,
  slug,
}: {
  title: string;
  date: string;
  slug: string;
}) {
  return (
    <>
      <div>
        <Link href={`/posts/${slug}`}>{title}</Link>
      </div>
      <p>
        {date}
      </p>
    </>
  );
}
