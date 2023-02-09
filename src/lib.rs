#![warn(missing_debug_implementations, rust_2018_idioms, unreachable_pub)]
#![deny(rustdoc::broken_intra_doc_links)]
#![doc(test(no_crate_inject, attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))))]

//! # rsnode
//!
//! TODO: docs

/// The client
pub mod client {
	pub use client::*;
}

/// The derivation pipeline
pub mod derivation {
	pub use derivation::*;
}

/// The backend database
pub mod backend {
	pub use backend::*;
}

/// Execution environment
pub mod execution {
	pub use execution::*;
}

/// Common rsnode types
pub mod common {
	pub use common::*;
}

/// A prelude of common types for easy usage of the [rsnode](crate) crate.
pub mod prelude {
	pub use super::backend::*;
	pub use super::client::*;
	pub use super::common::*;
	pub use super::derivation::*;
	pub use super::execution::*;
}
