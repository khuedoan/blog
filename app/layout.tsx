import './globals.css'

export const metadata = {
  title: 'Khue Doan',
  description: 'Khue\'s personal website',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  )
}
