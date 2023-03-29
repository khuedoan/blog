/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  experimental: {
    appDir: true,
  },
  images: {
    remotePatterns: [
      {
        protocol: 'https',
        hostname: 'archive.org',
        pathname: '/download/khuedoan-blog-images/**',
      },
    ],
  }
};

export default nextConfig;
