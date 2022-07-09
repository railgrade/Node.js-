# Overview

Crux is an **experimental** approach to building cross-platform applications with better testability, higher code and behaviour reuse, better safety, security, and more joy from better tools.

It splits the application into two distinct parts, a Core built in Rust, which drives as much of the business logic as possible, and a Shell, built in the platform native language (Swift, Kotlin, TypeScript), which provides all interfaces with the external world, including the human user, and acts as a platform on which the core runs.

![Crux](./crux.png)

The interface between the two is a native FFI (Foreign Function Interface) with cross-language type checking and message passing semantics, where simple data structures are passed across the boundary.

```admonish title="Get to know Crux"
To get playing with Crux quickly, follow the [Getting Started](./getting_started/core.md) steps. If you prefer to read more about how apps are built in Crux first, read the [Development Guide](./guide/hello_world.md). And if you'd like to know what possessed us to try this in the first place, r