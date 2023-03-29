import Link from "next/link";
import Image from "next/image";
import Date from "./date";

export default function PostPreview({
  slug,
  title,
  date,
  summary,
  coverImage,
  tags,
}: {
  slug: string;
  title: string;
  date: string;
  summary: string;
  coverImage: string;
  tags: string[];
}) {
  return (
    <article
      key={slug}
      className="relative isolate flex flex-col gap-8 lg:flex-row"
    >
      <div className="relative aspect-[16/9] lg:w-64 lg:shrink-0">
        <Link href={`/posts/${slug}`}>
          <Image
            className="absolute inset-0 h-full w-full rounded-2xl bg-gray-50 object-cover"
            src={coverImage}
            alt=""
            width="720"
            height="1280"
          />
          <div className="absolute inset-0 rounded-2xl ring-1 ring-inset ring-gray-900/10" />
        </Link>
      </div>
      <div>
        <div className="flex items-center gap-x-4 text-xs">
          <Date dateString={date} />
          {tags.map((tag: string) => (
            <Link
              key={tag}
              href={`/tags/${tag}`}
              className="relative z-10 rounded-full bg-gray-50 py-1.5 px-3 font-medium text-gray-600 hover:bg-gray-100"
            >
              {tag}
            </Link>
          ))}
        </div>
        <div className="group relative max-w-xl">
          <h2 className="mt-3 text-lg font-semibold leading-6 text-gray-900 group-hover:text-gray-600">
            <Link href={`/posts/${slug}`}>
              <span className="absolute inset-0" />
              {title}
            </Link>
          </h2>
          <p className="mt-5 text-sm leading-6 text-gray-600">{summary}</p>
        </div>
      </div>
    </article>
  );
}
