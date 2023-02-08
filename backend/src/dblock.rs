use std::collections::HashMap;

use crate::types::{BlockHash, BlockNumber, BlockWithReceipts, Database, DbResult};

/// Memory backend to store blocks and transactions.
#[derive(Debug, Default, Clone)]
pub struct DBlock<ExtDB: Database> {
	/// A map of block hash to the block object.
	pub blocks: HashMap<BlockHash, BlockWithReceipts>,
	/// A map from block number to block hash.
	pub hashes: HashMap<BlockNumber, BlockHash>,
	/// The internal database store.
	pub db: ExtDB,
}

impl<ExtDB: Database> DBlock<ExtDB> {
	/// Creates a new instance of [DBlock](crate::DBlock).
	pub fn new(db: ExtDB) -> Self {
		Self {
			blocks: HashMap::new(),
			hashes: HashMap::new(),
			db,
		}
	}
}

impl<ExtDB: Database> Database for DBlock<ExtDB> {
	/// Writes a block to the database storage.
	fn write_block(&mut self, block: BlockWithReceipts) -> DbResult<()> {
		let hash = block.block.hash.ok_or_else(|| eyre::eyre!("missing block hash"))?;
		let receipts = self.db.read_block(hash)?;
		let block_number = block.block.number.ok_or_else(|| eyre::eyre!("missing block number"))?;
		self.blocks.insert(
			hash,
			BlockWithReceipts {
				block: block.block,
				..receipts
			},
		);
		self.hashes.insert(block_number, hash);
		Ok(())
	}

	/// Reads a block from the database storage.
	fn read_block(&mut self, hash: BlockHash) -> DbResult<BlockWithReceipts> {
		if let Some(block) = self.blocks.get(&hash) {
			return Ok(block.clone());
		}
		let block = self.db.read_block(hash)?;
		self.blocks.insert(hash, block.clone());
		Ok(block)
	}
}
