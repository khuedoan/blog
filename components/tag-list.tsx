import { getAllTags } from "@/lib/post";

export default function TagList() {
  const tags = getAllTags();

  return (
    <ul
      role="list"
      className="mt-3 grid grid-cols-1 gap-5 sm:grid-cols-2 sm:gap-6 lg:grid-cols-4"
    >
      {tags.map((tag) => (
        <li key={tag} className="col-span-1 rounded-md shadow-sm">
          <a href={`/tags/${tag}`}>
            <div className="flex flex-1 items-center justify-between truncate rounded-md border border-gray-200 bg-white">
              <div className="flex-1 truncate px-4 py-2 text-sm">
                <p className="font-medium text-gray-900 hover:text-gray-600">
                  <code>{tag}</code>
                </p>
              </div>
            </div>
          </a>
        </li>
      ))}
    </ul>
  );
}
