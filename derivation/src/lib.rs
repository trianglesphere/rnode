#![warn(missing_debug_implementations, rust_2018_idioms, unreachable_pub)]
#![deny(rustdoc::broken_intra_doc_links)]
#![doc(test(no_crate_inject, attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))))]

//! # Derivation
//!
//! The derivation pipeline derives the state of the rollup chain from the L1 chain.
//!
//! TODO: more docs + examples

/// The System Config
pub mod sys_config;

/// Derivation Frames
pub mod frames;

/// Derivation Channels
pub mod channels;

/// Derivation Channel Bank
pub mod channel_bank;

/// Batches;
pub mod batches;

/// A prelude of derivation pipeline types
pub mod prelude {
	pub use super::batches::*;
	pub use super::channel_bank::*;
	pub use super::channels::*;
	pub use super::frames::*;
	pub use super::sys_config::*;
}
