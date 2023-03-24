export default function Footer() {
  return (
    <footer>
      <div className="mx-auto max-w-7xl overflow-hidden py-20 px-6 sm:py-24 lg:px-8">
        <p className="mt-5 text-center text-sm leading-5 text-gray-500">
          &copy; {new Date().getFullYear()} Khue Doan. All rights reserved.
        </p>
      </div>
    </footer>
  );
}
