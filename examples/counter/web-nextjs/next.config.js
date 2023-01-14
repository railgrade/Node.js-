const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const path = require("path");

// see https://github.com/wasm-tool/wasm-pack-plugin/issues/112
let loaded = false;

/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: false,
  webpack(nextConfig) {
    if (!loaded) {
      nextConfig.plugins.push(
        new WasmPackPlugin({
          crateDirectory: path.resolve(__dirname, "..", "shared"),
          extraArgs: "--target web",
          outDir: path.resolve(__dirname, "shared", "core"),
        })
      );
      loaded = true;
    }

    return nextConfig;
  },
  async headers() {
    return [
      {
        source: "/(.*)",
        headers: [
          {
            key: "Content-Security-Policy",
            value:
              "default-src 'self' 'unsafe-inline' 'unsafe-eval' crux-counter.fly.dev cdn.jsdelivr.net",
          },
          {
            key: "Referrer-Policy",
            value: "strict-origin-when-cross-origin",
          },
        ],
      },
    ];
  },
};

module.exports = nextConfig;
