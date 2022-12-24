# Crux Counter Example

The Crux Counter example is a simple multi-platform application that calls a cloud-hosted API.

It makes HTTP requests to a shared global counter hosted at [https://crux-counter.fly.dev](https://crux-counter.fly.dev), incrementing or decrementing the counter value.

The [server](./server/) also has an endpoint for Server Sent Events ([https://crux-counter.fly.dev/sse](https://crux-counter.fly.dev/sse)), which signals when changes are made to the global counter value — so that when you update the counter in one client, all the other clients will update too.

We have included an example of a [Server Sent Events capability](./shared/src/capabilities/sse.rs), which uses the core's ability to stream responses from the shell.

![screenshots](./counter.webp)

## Rust

1. Make sure you have the following rust targets installed (there is a [`rust-toolchain.toml`](../../rust-toolchain.toml) in the root directory of this repo, so you should be able to type `rustup target list --installed`, in or below the root directory, and these targets will be installed if they are not already present).

   ```txt
   aarch64-apple-darwin
   aarch64-apple-ios
   aarch64-apple-ios-sim
   aarch64-linux-android
   wasm32-unknown-unknown
   x86_64-apple-ios
   ```

1. Make sure the core builds

   ```sh
   cd shared
   cargo build
   ```

1. Generate the shared types for your client applications

   ```sh
   cd ../shared_types
   cargo build
   ```

## Yew Web app

The web application should now build and run

```
cd web-yew
trunk serve
```

## React Web app

The web application shoul