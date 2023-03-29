import Date from "./date";

export default function PostHeader({
  title,
  date,
}: {
  title: string;
  date: string;
}) {
  return (
    <div className="py-20 text-center">
      <p className="text-base font-semibold leading-7 text-indigo-600">
        <Date dateString={date} />
      </p>
      <h1 className="mt-2 text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">
        {title}
      </h1>
    </div>
  );
}
