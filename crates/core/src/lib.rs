pub mod chain_config;
pub mod id;
pub mod types;

use ethers_core::types::Transaction;

#[derive(Debug)]
pub struct L2BlockCandidate {
	pub number: u64,
	pub timestamp: u64,
	pub transactions: Vec<Transaction>,
	// TODO: tx root
}

pub mod prelude {
	pub use crate::chain_config::RollupConfig;
	pub use crate::id::BlockID;
	pub use crate::id::L1BlockRef;
	pub use crate::id::L2BlockRef;
	pub use crate::types::Address;
	pub use crate::types::ChannelID;
	pub use crate::types::Hash;
	pub use crate::types::Receipt;
	pub use crate::types::Transaction;
	pub use crate::L2BlockCandidate; // TODO: remove
}
