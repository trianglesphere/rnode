use ethers_core::types::{Block, Transaction, TransactionReceipt};
use ethers_core::types::{H256, U64};
use serde::{Deserialize, Serialize};

// TODO: remove this and use upper-level type
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockWithReceipts {
	/// Block
	pub block: Block<Transaction>,
	/// Block Receipts
	pub receipts: Vec<TransactionReceipt>,
}

/// A Database read-write result.
pub type DbResult<T> = eyre::Result<T>;

pub type BlockHash = H256;
pub type BlockNumber = U64;

/// Database is a trait that defines the interface for a persistent, on-disk database store.
pub trait Database {
	/// Writes a block to the database storage.
	fn write_block(&mut self, block: BlockWithReceipts) -> DbResult<()>;
	/// Reads a block from the database storage.
	fn read_block(&mut self, hash: BlockHash) -> DbResult<BlockWithReceipts>;
}
