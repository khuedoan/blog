import Article from "@/components/article";

export default function PostLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <>
      <Article>{children}</Article>
    </>
  );
}
