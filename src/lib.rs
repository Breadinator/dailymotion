#![warn(clippy::pedantic)]

/// Dailymotion `OAuth2`
pub mod auth;

#[macro_use]
pub mod api;

#[macro_use]
mod scopes;
pub use scopes::*;

mod env;
pub use env::EnvironmentVariable;

mod err;
pub use err::{
    Error,
    Result,
};

