# Web — TypeScript and React (Next.js)

These are the steps to set up and run a simple TypeScript Web app that calls into a shared core.

```admonish
This walk-through assumes you have already added the `shared` and `shared_types` libraries to your repo, as described in [Shared core and types](./core.md).
```

```admonish info
There are many frameworks available for writing Web applications with JavaScript/TypeScript. We've chosen [React](https://reactjs.org/) with [Next.js](https://nextjs.org/) for this walk-through because it is simple and popular. However, a similar setup would work for other frameworks.
```

## Create a Next.js App

For this walk-through, we'll use the [`pnpm`](https://pnpm.io/) package manager for no reason other than we like it the most!

Let's create a simple Next.js app for TypeScript, using `pnpx` (from `pnpm`). You can accept all the defaults.

```sh
pnpx create-next-app@latest web-nextjs --ts --use-pnpm
```

Then we can change to the project directory and start adding dependencies.

```sh
cd web-nextjs
```

## Compile our Rust shared library

When we build our app, we also want to compile the Rust core to WebAssembly so that it can be referenced from our code.

To do this, we'll use [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/), which you can install like this:

```sh
# with homebrew
brew install wasm-pack

# or directly
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

But we won't call `wasm-pack` directly. Instead we'll use a Next.js plugin to do this for us. That way the shared library will be compiled to WebAssembly when we build our app using `pnpm`.

Add the `wasm-pack-plugin` to our project:

```sh
pnpm install --save-dev @wasm-tool/wasm-pack-plugin
```

We'll need to configure Next.js to call the plugin by editing `web-nextjs/next.config.js` to look like this:

```javascript
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const path = require("path");

// see https://github.com/wasm-tool/wasm-pack-plugin/issues/112
let loaded = false;

/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  webpack(nextConfig) {
    if (!loaded) {
      nextConfig.plugins.push(
        new WasmPackPlugin({
          crateDirectory: path.resolve(_