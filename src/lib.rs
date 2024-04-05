#![forbid(unsafe_code)]
#![allow(clippy::module_inception)]
#![deny(clippy::expect_used, clippy::unwrap_used)]
//
pub mod config;
pub mod constants;
pub mod error;
pub mod routes;
pub mod telemetry;
