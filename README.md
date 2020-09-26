# Crux — Cross-platform app development in Rust

<p>
  <a href="https://crates.io/crates/crux_core"><img alt="Crate Info" src="https://img.shields.io/crates/v/crux_core.svg"/></a>
  <a href="https://docs.rs/crux_core/"><img alt="API Docs" src="https://img.shields.io/badge/docs.rs-crux_core-green"/></a>
</p>

Crux helps you share your app's business logic and behavior across mobile (iOS and Android) and web, as a single, reusable core built with Rust.

Unlike [React Native](https://reactnative.dev/), but like [Kotlin Multi-platform Mobile](https://kotlinlang.org/lp/mobile/), the user interface layer is built natively, with modern declarative UI frameworks such as [SwiftUI](https://developer.apple.com/xcode/swiftui/), [Jetpack Compose](https://developer.android.com/jetpack/compose) and [React](https://reactjs.org/)/[Vue](https://vuejs.org/) or a Wasm based framework (like [Yew](https://yew.rs/)) on the web.

The UI layer is as thin as it can be, and all other work is done by the shared core. The interface with the core has static type checking across languages.

> Note, that Crux is experimental and currently under active development (probably not ready for use in production apps just yet). However, the master branch should always be working well, and we will try to keep the examples and documentation up to date as we go. We _do_ think that the API has now settled, so have a play! :-)

This readme describes at a high level how Crux works, but you can find more details in the [book](https://redbadger.github.io/crux).

You can also join the friendly conversation on our [Zulip channel](https://crux-community.zulipchat.com).

# Architectural Overview

![Logical architecture](./crux_core/architecture.svg)

The fundamental architectural concept is the strict separation of pure computational tasks from tasks that cause side effects.
This is similar to the way [Elm](https://guide.elm-lang.org/architecture/) works.

### Side-effect-free core

In the above diagram, the inner "Core" is compiled and linked to the outer "Shell" on each platform as a library:

- On iOS as a native static library
- On Android as a dynamic library using [Java Native Access](https://github.com/java-native-access/jna)
- In a browser as a WebAssembly module

In fact, because WebAssembly (Wasm) is one of the compilation targets, the core _must_ remain side-effect free, due to the sandboxed nature of the Wasm runtime environment.

As such, the core is completely isolated and secure against software supply-chain attacks, as it has
no access to any external APIs.
All it can do is perform pure calculations and keep internal state.

Following the Elm architecture, the core defines the key component types within the application:

- `Event` — an `enum` describing the events which the core can ha