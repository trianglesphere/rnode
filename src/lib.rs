#![warn(missing_debug_implementations, rust_2018_idioms, unreachable_pub)]
#![deny(rustdoc::broken_intra_doc_links)]
#![doc(test(no_crate_inject, attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))))]

//! # rsnode
//!
//! TODO: docs

/// The backend database
pub mod backend {
	pub use backend::*;
}

/// The client module
pub mod client;

/// The oracle module
pub mod oracle;

/// Common Types
pub mod types;

/// The execution environment
pub mod execution {
	pub use execution::*;
}

/// A prelude of common types for easy usage of the [rsnode](crate) crate.
pub mod prelude {
	pub use super::backend::*;
	pub use super::client::*;
	pub use super::oracle::*;
	pub use super::types::*;
}
