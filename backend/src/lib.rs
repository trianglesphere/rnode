#![warn(missing_debug_implementations, rust_2018_idioms, unreachable_pub)]
#![deny(rustdoc::broken_intra_doc_links)]
#![doc(test(no_crate_inject, attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))))]

//! # Revm Database Implementation
//!
//! This crate exposes a database implementation for reading and writing
//! [BlockWithReceipts](crate::types::BlockWithReceipts) to persistent storage.
//!
//! Uses [revm](https://github.com/bluealloy/revm)'s [Database](revm::Database) trait.
//!
//! ## Example
//!
//! ```rust
//! use backend::dblock::{DBlock};
//! use backend::benchdb::{BenchDb};
//! use backend::types::{BlockHash, BlockNumber, Database};
//! use ethers_core::types::{H256};
//! use ethers_core::abi::AbiDecode;
//!
//! let block_number = BlockNumber::from(42);
//! let block_hash: BlockHash = H256::decode_hex("1d2b0bda21d56b8bd12d4f94ebacffdfb35f5e226f84b461103bb8beab6353be").unwrap();
//! let mut database = DBlock::new(BenchDb::default());
//! let block = database.read_block(block_hash).unwrap();
//! assert_eq!(block.block.number, Some(BlockNumber::from(0)));
//! ```

/// A block database.
pub mod dblock;

/// Common types
pub mod types;

/// A benchmark database.
pub mod benchdb;

/// Prelude of common types for easy usage of the [backend](crate) crate.
pub mod prelude {
	pub use super::benchdb::*;
	pub use super::dblock::*;
	pub use super::types::*;
}
