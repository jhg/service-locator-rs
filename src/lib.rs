#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

// This is for the guards not be in a child module, and then
// can not access the private fields or methods of the ServiceLocator.
mod locator;
pub use locator::ServiceLocator;

mod guard;
pub use guard::{ServiceReadGuard, ServiceWriteGuard};

mod error;
pub use error::ServiceLocatorError;
