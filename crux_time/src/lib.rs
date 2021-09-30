//! Current time access for Crux apps
//!
//! Current time (on a wall clock) is considered a side-effect (although if we were to get pedantic, it's
//! more of a side-cause) by Crux, and has to be obtained externally. This capability provides a simple
//! interface to do so.
//!
//! This is still work in progress and as such very basic. It returns time as an IS08601 string.
use crux_core::{
    capability::{CapabilityContext, Operation},
    Capability,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeRequest;

// TODO revisit this
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeResponse(pub String);

impl Operation for TimeRequest {
    type Output = T