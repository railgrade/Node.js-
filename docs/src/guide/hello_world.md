# Hello world

As the first step, we will build a simple application, starting with a classic Hello World, adding some state, and finally a remote API call. We will focus on the core, rely on tests to tell us things work, and return to the shell a little later, so unfortunately there won't be much to see until then.

If you want to follow along, you should start by following the [Shared core and types](../getting_started/core.md), guide to set up the project.

## Creating an app

To start with, we need a `struct` to be the root of our app.

```rust,noplayground
#[derive(Default)]
pub struct Hello;
```

We need to implement `Default` so that Crux can construct the app for us.

To turn it into an app, we need to implement the `App` trait from the `crux_core` crate.

```rust,noplayground
use crux_core::App;

#[derive(Default)]
pub struct Model;

impl App for Hello {
```

If you're following along, the compiler is now screaming at you that you're missing four associated types for the trait: `Event`, `Model`, `ViewModel` and `Capabilities`.

Capabilities is the more complicated of them, and to understand what it does, we need to talk about what