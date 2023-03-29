import PostList from "@/components/post-list";

export default function Posts() {
  return (
    <>
      <div className="text-center">
        <h1 className="mt-2 py-20 text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">
          Posts
        </h1>
      </div>
      <PostList />
    </>
  );
}
