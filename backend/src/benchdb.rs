use crate::types::{BlockHash, BlockNumber, BlockWithReceipts, Database, DbResult};

use ethers_core::types::{Block, Transaction, TransactionReceipt};

/// Benchmark backend that records reads and writes.
#[derive(Debug, Clone, Default)]
pub struct BenchDb {
	/// A counter for writes.
	pub writes: usize,
	/// A counter for reads.
	pub reads: usize,
}

impl BenchDb {
	/// Creates a new instance of [BenchDb](crate::BenchDb).
	pub fn new() -> Self {
		Self { writes: 0, reads: 0 }
	}
}

impl Database for BenchDb {
	/// Mocks a write to the database storage.
	fn write_block(&mut self, _block: BlockWithReceipts) -> DbResult<()> {
		self.writes += 1;
		Ok(())
	}

	/// Mocks a read from the database storage.
	fn read_block(&mut self, hash: BlockHash) -> DbResult<BlockWithReceipts> {
		self.reads += 1;
		Ok(BlockWithReceipts {
			block: Block {
				hash: Some(hash),
				number: Some(BlockNumber::from(0)),
				transactions: vec![Transaction {
					hash,
					..Default::default()
				}],
				..Default::default()
			},
			receipts: vec![TransactionReceipt {
				transaction_hash: hash,
				..Default::default()
			}],
		})
	}
}
