use super::batch::*;
use super::batch_queue::*;
use super::channel_bank::*;
use super::frame::parse_frames;
use super::read_adapter::ReadAdpater;

use core::prelude::*;

use core::types::{Receipt, Transaction};
use ethers_core::utils::rlp::{decode, Rlp};
use flate2::read::ZlibDecoder;
use std::io::Read;

fn decompress(r: impl Read) -> Vec<u8> {
	let mut decomp = ZlibDecoder::new(r);
	let mut buffer = Vec::default();

	// TODO: Handle this error
	// Decompress the passed data with zlib
	decomp.read_to_end(&mut buffer).unwrap();
	buffer
}

fn parse_batches(data: Vec<u8>) -> Vec<Batch> {
	// TODO: Truncate data to 10KB (post compression)
	// The data we received is an RLP encoded string. Before decoding the batch itself,
	// we need to decode the string to get the actual batch data.
	let mut decoded_batches: Vec<Vec<u8>> = Vec::new();
	let mut buf: &[u8] = &data;

	loop {
		let rlp = Rlp::new(buf);
		let size = rlp.size();

		match rlp.as_val() {
			Ok(b) => {
				decoded_batches.push(b);
				buf = &buf[size..];
			}
			Err(_) => break,
		}
	}
	// dbg!(decoded_batches);
	decoded_batches.iter().filter_map(|b| decode(b).ok()).collect()
}

#[derive(Default, Debug)]
pub struct Derivation {
	channel_bank: ChannelBank,
	batch_queue: BatchQueue,
}

impl Derivation {
	pub fn load_l1_data(&mut self, l1_block: L1BlockRef, transactions: Vec<Transaction>, _receipts: Vec<Receipt>) {
		// TODO: Create system config from the receipts
		let batcher_address = hex_literal::hex!("7431310e026B69BFC676C0013E12A1A11411EEc9").into();

		let batches = transactions
			.into_iter()
			.filter(move |tx| tx.from == batcher_address)
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
