const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const path = require("path");

/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: false,
  swcMinify: true,
};

// see https://github.com/wasm-tool/wasm-pack