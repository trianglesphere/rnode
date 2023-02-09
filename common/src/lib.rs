use ethers_core::types::{Block, Transaction, TransactionReceipt};
use serde::{Deserialize, Serialize};

/// A block with associated receipts.
#[derive(Serialize, Deserialize, Debug)]
pub struct BlockWithReceipts {
	/// The block
	pub block: Block<Transaction>,
	/// The block's receipts
	pub receipts: Vec<TransactionReceipt>,
}
