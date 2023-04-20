use ethers_core::{types::Transaction, utils::rlp::decode};
use std::collections::{HashMap, VecDeque};

use super::batch::Batch;
use core::prelude::*;

#[derive(Debug)]
pub struct BatchQueue {
	l1_blocks: VecDeque<L1BlockRef>,
	// Map batch timestamp to batches in order that they were received
	batches: HashMap<u64, VecDeque<Batch>>,

	l2_block_time: u64,
	// seq_window_size: u64,
	// max_sequencer_drift: u64,
}

impl BatchQueue {
	pub fn new(cfg: RollupConfig) -> Self {
		BatchQueue {
			l1_blocks: VecDeque::default(),
			batches: HashMap::default(),
			l2_block_time: cfg.l2_block_time,
			// seq_window_size: cfg.seq_window_size,
			// max_sequencer_drift: cfg.max_sequencer_drift,
		}
	}
	pub fn load_batches(&mut self, batches: impl Iterator<Item = Batch>, l1_origin: L1BlockRef) {
		self.l1_blocks.push_back(l1_origin);
		for b in batches {
			self.batches.entry(b.batch.timestamp).or_default().push_back(b);
		}
	}

	pub fn get_block_candidate(&mut self, l2_head: L2BlockRef) -> Option<L2BlockCandidate> {
		let next_timestamp = l2_head.time + self.l2_block_time;
		if let Some(candidates) = self.batches.get(&next_timestamp) {
			#[allow(clippy::never_loop)]
			for b in candidates {
				// TODO: Do this step earlier
				let txns = b.batch.transactions.iter().map(|t| decode::<Transaction>(t).unwrap()).collect();
				self.batches.remove(&next_timestamp);
				// TODO: deposits, seq number, transactions from batches
				return Some(L2BlockCandidate {
					number: l2_head.number + 1,
					timestamp: next_timestamp,
					transactions: txns,
				});
			}
		}

		None
	}
}
