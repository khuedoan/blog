export default function PostHeader({
  title,
  date,
}: {
  title: string;
  date: string;
}) {
  return (
    <>
      <h1>{title}</h1>
      <p>{date}</p>
    </>
  );
}
