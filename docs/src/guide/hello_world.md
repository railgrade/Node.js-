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

Capabilities is the more complicated of them, and to understand what it does, we need to talk about what makes Crux different from most UI frameworks.

## Side-effects and capabilities

One of the key design choices in Crux is that the Core is free of side-effects (besides its internal state). Your application can never _perform_ anything that directly interacts with the environment around it - no network calls, no reading/writing files, and (somewhat obviously) not even updating the screen. Actually _doing_ all those things is the job of the Shell, the core can only _ask_ for them to be done.

This makes the core portable between platforms, and, importantly, really easy to test. It also separates the intent, the "functional" requirements, from the implementation of the side-effects and the "non-functional" requirements (NFRs). For example, your application knows it wants to store data in a SQL database, but it doesn't need to know or care whether that database is local or remote. That decision can even change as the application evolves, and be different on each platform. If you want to understand this better before we carry on, you can read a lot more about how side-effects work in Crux in the chapter on [capabilities](./capabilities.md).

To _ask_ the Shell for side effects, it will need to know what side effects it needs to handle, so we will need to declare them (as an enum). _Effects_ are simply messages describing what should happen, and for more complex side-effects (e.g. HTTP), they would be too unwieldy to create by hand, so to help us create them, Crux provides _capabilities_ - reusable libraries which give us a nice A