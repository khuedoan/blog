import TagList from "@/components/tag-list";

export default function Tags() {
  return (
    <div className="mx-auto max-w-3xl px-6 lg:px-8">
      <h1 className="mt-2 py-20 text-center text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">
        Tags
      </h1>
      <TagList />
    </div>
  );
}
