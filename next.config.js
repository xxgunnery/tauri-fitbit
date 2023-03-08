const { default: next } = require('next');

/** @type {import('next').NextConfig} */
const isProd = process.env.NODE_ENV === "production";
module.exports = async (phase, { defaultConfig }) => {
  let internalHost = null;
  if (!isProd) {
    const os = await import("os");
    const interfaces = await os.networkInterfaces();
    internalHost = await interfaces["Ethernet"].filter((interface) => interface.family === "IPv4")[0].address;
  }

  const nextConfig = {
    reactStrictMode: true,
    reactStrictMode: true,
    swcMinify: true,
    // Note: This experimental feature is required to use NextJS Image in SSG mode.
    // See https://nextjs.org/docs/messages/export-image-api for different workarounds.
    images: {
      unoptimized: true,
    },
    assetPrefix: isProd ? null : `http://${internalHost}:3000`,
  };

  return nextConfig;
};
