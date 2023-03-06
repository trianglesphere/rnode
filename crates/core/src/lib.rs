pub mod id;
pub mod types;

use ethers_core::types::Transaction;

#[derive(Debug)]
pub struct L2BlockCandidate {
	// pub parent_hash: H256,
	pub number: u64,
	pub timestamp: u64,
	pub transactions: Vec<Transaction>,
	// TODO: tx root
}

pub mod prelude {
	pub use crate::id::BlockID;
	pub use crate::id::L1BlockRef;
	pub use crate::id::L2BlockRef;
	pub use crate::types::Address;
	pub use crate::types::Receipt;
	pub use crate::types::Transaction;
	pub use crate::types::H128;
	pub use crate::types::H256;
	pub use crate::L2BlockCandidate; // TODO: remove
}
