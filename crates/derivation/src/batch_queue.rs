use ethers_core::{types::Transaction, utils::rlp::decode};
use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};

use super::batch::Batch;
use core::prelude::*;

#[derive(Default, Debug)]
pub struct BatchQueue {
	l1_blocks: VecDeque<L1BlockRef>,
	// Map batch timestamp to batches in order that they were received
	batches: HashMap<u64, VecDeque<Batch>>,
}

const L2_BLOCK_TIME: u64 = 2u64;
// const SEQ_WINDOW_SIZE: u64 = 3600u64;

impl BatchQueue {
	pub fn load_batches(&mut self, batches: impl Iterator<Item = Batch>, l1_origin: L1BlockRef) {
		self.l1_blocks.push_back(l1_origin);
		for b in batches {
			println!("{b:?}");
			if let Entry::Vacant(e) = self.batches.entry(b.batch.timestamp) {
				e.insert(VecDeque::default());
			}
			self.batches.get_mut(&b.batch.timestamp).unwrap().push_back(b);
		}
	}

	pub fn get_block_candidate(&mut self, l2_head: L2BlockRef) -> Option<L2BlockCandidate> {
		let next_timestamp = l2_head.time + L2_BLOCK_TIME;
		if let Some(candidates) = self.batches.get(&next_timestamp) {
			let out = candidates.front().expect("Should have entry in any created queue");
			// TODO: Throw out the batch if we can't decode it.
			let txns = out.batch.transactions.iter().map(|t| decode::<Transaction>(t).unwrap()).collect();
			self.batches.remove(&next_timestamp);
			// TODO: deposits, seq number, transactions from batches
			return Some(L2BlockCandidate {
				number: l2_head.number + 1,
				timestamp: next_timestamp,
				transactions: txns,
			});
		}

		None
	}
}
