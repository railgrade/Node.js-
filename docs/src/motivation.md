# Motivation

We set out to prove this approach to building apps largely because we've seen the drawbacks of all the other approaches in real life, and thought "there must be a better way". The two major available approaches to building the same application for iOS and Android are:

1. Build a native app for each platform, effectively doing the work twice.
2. Use React Native or Flutter to build the application once[^once] and produce native looking and feeling apps which behave nearly identically.

The drawback of the first approach is doing the work twice. In order to build every feature for iOS and Android at the same time, you need twice the number of people, either people who happily do Swift and Kotlin (and they are very rare), or more likely a set of iOS engineers and another set of Android engineers. This typically leads to forming two separate, platform-focused teams. We have witnessed situations first-hand, where those teams struggle with the same design problems, and despite one encountering and solving the problem first, the other one can learn nothing from their experience (and that's _despite_ long design discussions).

We think such experience with the platform native approach are common, and the reason why people look to React Native and Flutter. The issues with React Native are two fold

- Only _mostly_ native user interface
- In the case of React Native, the JavaScript ecosystem tooling disaster

React Native effectively takes over, and works hard to insulate th