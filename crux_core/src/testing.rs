//! Testing support for unit testing Crux apps.
use std::{fmt, rc::Rc};

use crate::{
    capability::CapabilityContext,
    channels::Receiver,
    executor::{executor_and_spawner, QueuingExecutor},
    steps::StepRegistry,
    Request, Step, WithContext,
};

/// AppTester is a simplified execution environment for Crux apps for use in
/// tests.
///
/// Create an instance of `AppTester` with your `App` and an `Effect` type
/// using [`AppTester::default`].
///
/// for example:
///
/// ```rust,ignore
/// let app = AppTester::<ExampleApp, ExampleEffect>::default();
/// ```
pub struct AppTeste