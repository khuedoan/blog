import Footer from "@/components/footer";
import Header from "@/components/header";
import "@/styles/globals.css";

export const metadata = {
  title: "Khue Doan",
  description: "Khue's personal website",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>
        <Header />
        <main>{children}</main>
        <Footer />
      </body>
    </html>
  );
}
