# Capabilities

In the last chapter, we spoke about Effects. In this one we'll look at the APIs your app will actually use to request them – the capabilities.

Capabilities are reusable, platform agnostic APIs for a particular type of effect. They have two key jobs:

1. Provide a nice ergonomic API for apps to use
2. Manage the communication between the app and the Shell

From the perspective of the app, you can think of capabilities as an equivalent to SDKs. And a lot of them will provide an interface to the actual platform specific SDKs.

## Intent and execution

The Capabilities are the key to Crux being portable across as many platforms as is sensible. Crux apps are, in a sense, built in the abstract, they describe _what_ should happen in response to events, but not _how_ it should happen. We think this is important both for portability, and for testing and general separation of concerns. What should happen is inherent to the product, and should behave the same way on any platform – it's part of what your app _is_. How it should be executed (and exactly what it looks like) often depends on the platform.

Different platforms may support different ways, for example a biometric authentication may work very differently on various devices and some may not even support it at all, but it may also be a matter of convention. Different platforms may also have different practical restrictions: while it may be perfectly appropriate to write things to disk on one platform, but internet access can't be guaranteed (e.g. on a smart watch), on another, writing to disk may not be possible, but internet connection is virtually guaranteed (e.g. in an API service, or on an embedded device in a factory). A persistent caching capability would implement the specific storage solution differently on different platforms, but would potentially share the key format and eviction strategy across them. The hard part of designing a capability is working out exactly where to draw the line between what is the intent and what is the implementation detail, what's common across platforms and what may be different on each, and implementing the former in Rust in the capability and the latter on the native side in the Shell, however is appropriate.

Because Capabilities can own the "language" used to express intent, and the interface to request the execution of the effect, your Crux application code can be portable onto any platform capable of executing the effect in some way. Clearly, the number of different effects we can think of, and platforms we can target is enormous, and Crux doesn't want to force you to implement the entire portfolio of them on every platform. That's why Capabilities are delivered as separate modules, typically in crates, and apps can declare which ones they need. The Shell implementations need to know how to handle all requests from those capabilities, but can choose to provide only stub implementations where appropriate. For example the [Cat Facts example](https://github.com/redbadger/crux/tree/master/examples/cat_facts), uses a key-value store capability for persisting the model after every interaction, which is crucial to make the CLI shell work statefully, but the other shells generally ignore the key-value requests, because state persistence across app launches is not crucial for them. The app itself (the Core) has no idea which is the case.

In some cases, it may also make sense to implement an app-specific capability, for effects specific to your domain, which don't have a common implementation across platforms (e.g. registering a local user). Crux does not stop you from bundling a number of capabilities alongside your apps (i.e. they don't _have to_ come from a crate). On the other hand, it might make sense to build a capability on top of an existing lower-level capability, for example a CRDT capability may use a general pub/sub capability as transport, or a specific protocol to speak to your synchronization server (e.g. over HTTP).

There are clearly numerous scenarios, and the best rule of thumb we can think of is "focus on the intent". Provide an API to describe the intent of side-effects and then either pass the intent straight to the shell, or translate it to a sequence of more concrete intents for the Shell to execute. And keep in mind that the more complex the intent sent to the shell, the more complex the implementation on each platform. The translation between high-level intent and low level building blocks is why Capabilities exist.

## The Core and the Shell

As we've already covered, the capabilities effectively straddle the FFI boundary between the Core and the Shell. On the Core side they mediate between the FFI boundary and the application code. On the shell-side the requests produced by the capability need to be actually executed and fulfilled. Each capability therefore extends the Core/Shell interface with a set of defined (and type checked) messages, in a way that allows Crux to leverage exhaustive pattern matching on the native side to ensure all necessary capabilities required by the Core are implemented.

At the moment the Shell implementation is up to you, but we think in the future it's likely that capability crates will come with platform native code as well, making building both the Core and the Shells easier, and allow you to focus on application behaviour in the Core and look and feel in the Shell.

## Using Capabilities

Okay, time to get practical. We'll look at what it takes (and why) to use a capability, and in the next couple of chapters, we'll continue to build one and implement the Shell side of it.

Firstly, we need to have access to an instance of the capability in our `update` function. Recall that the function signature is:

```rust,noplayground
fn update(&self, msg: Self::Event, model: &mut Self::Model, caps: &Self::Capabilities)
```

We get the capabilities in the `caps` argument. You may be wondering why that's necessary. At first glance, we could be able to just create a capability instance ourselves, or not need one at all, after all they just provide API to make effects. There are a few reasons.

Firstly, capabilities need to be able to send a message to the shell, more precisely, they need to be able to add to the set of effects which result from the run of the update function. Sounds like a return value to you? It kind of is, and we tried that, and the type signatures involved quickly become quite unsightly. It's not the only reason though. They also need t