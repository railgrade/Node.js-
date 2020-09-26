# Crux — Cross-platform app development in Rust

<p>
  <a href="https://crates.io/crates/crux_core"><img alt="Crate Info" src="https://img.shields.io/crates/v/crux_core.svg"/></a>
  <a href="https://docs.rs/crux_core/"><img alt="API Docs" src="https://img.shields.io/badge/docs.rs-crux_core-green"/></a>
</p>

Crux helps you share your app's business logic and behavior across mobile (iOS and Android) and web, as a single, reusable core built with Rust.

Unlike [React Native](https://reactnative.dev/), but like [Kotlin Multi-platform Mobile](https://kotlinlang.org/lp/mobile/), the user interface layer is built natively, with modern declarative UI frameworks such as [SwiftUI](https://developer.apple.com/xcode/swiftui/), [Jetpack Compose](https://developer.android.com/jetpack/compose) and [React](https://reactjs.org/)/[Vue](https://vuejs.org/) or a Wasm based framework (like [Yew](https://yew.rs/)) on the web.

The UI layer is as thin as it can be, and all other work is done by the shared core. The interface with the core has static type checking across languages.

> Note, that Crux is experimental and currently under active development (probably not ready for use in production apps ju