//! Common traits and logic for managing the lifecycle of services
#![deny(unused_crate_dependencies)]
#![deny(missing_docs)]
#![deny(warnings)]

extern crate core;

pub mod block_verifier;
pub use block_verifier::config::RelayerVerifierConfig;
