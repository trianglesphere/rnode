use crate::batch::parse_batches;
use crate::batch_queue::*;
use crate::channel_bank::*;
use crate::frame::parse_frames;
use crate::read_adapter::ReadAdpater;

use core::prelude::*;

use flate2::read::ZlibDecoder;
use std::io::Read;

#[derive(Debug)]
pub struct Derivation {
	channel_bank: ChannelBank,
	batch_queue: BatchQueue,
	config: RollupConfig,
}

impl Derivation {
	pub fn new(cfg: RollupConfig) -> Self {
		Self {
			channel_bank: ChannelBank::new(cfg),
			batch_queue: BatchQueue::new(cfg),
			config: cfg,
		}
	}
	pub fn load_l1_data(&mut self, l1_block: L1BlockRef, transactions: Vec<Transaction>, _receipts: Vec<Receipt>) {
		// TODO: update system config from receipts

		let sys_config = self.config.system_config;

		let batches = transactions
			.into_iter()
			.filter(|tx| tx.to == Some(self.config.batch_inbox_address))
			.filter(|tx| tx.from == sys_config.batcher_address)
			.flat_map(|tx| parse_frames(&tx.input))
			.reassemble_channels(&mut self.channel_bank, l1_block.into())
			.map(|c| c.data())
			.map(ReadAdpater::new)
			.map(decompress)
			.flat_map(parse_batches);
		self.batch_queue.load_batches(batches, l1_block);
	}

	pub fn next_l2_attributes(&mut self, l2_head: L2BlockRef) -> Option<L2BlockCandidate> {
		self.batch_queue.get_block_candidate(l2_head)
	}
}

fn decompress(r: impl Read) -> Vec<u8> {
	let mut decomp = ZlibDecoder::new(r);
	let mut buffer = Vec::default();

	// TODO: Handle this error
	// Decompress the passed data with zlib
	decomp.read_to_end(&mut buffer).unwrap();
	buffer
}
