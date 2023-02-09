#![allow(dead_code)]

use dotenv::dotenv;
use ethers_core::{
	types::{Address, Transaction, TransactionReceipt, H128, H256},
	utils::rlp::{decode, Rlp},
};
use eyre::Result;
use flate2::read::ZlibDecoder;
use std::io::Read;
use std::{
	collections::{HashMap, VecDeque},
	str::FromStr,
};

/// The client module
pub mod client;
pub use client::*;

// Module Types
pub mod types;
pub use types::*;

// MPT utils
pub mod mpt;
pub use mpt::*;

// Module Data
pub mod data;
pub use data::*;
// MPT utils
pub mod mpt;
pub use mpt::*;

// ConfigUpdateEventABI      = "ConfigUpdate(uint256,uint8,bytes)"
// ConfigUpdateEventABIHash  = crypto.Keccak256Hash([]byte(ConfigUpdateEventABI))
// ConfigUpdateEventVersion0 = common.Hash{}

#[derive(Default)]
struct Channel {
	frames: HashMap<u16, Frame>,
	// TODO: Size + handling insertion of frames
}

#[derive(Default)]
struct ChannelBank {
	channels_map: HashMap<H128, Channel>,
	channels_by_creation: VecDeque<H128>,
	// TODO: Pruning
}

impl Channel {
	pub fn load_frame(&mut self, frame: Frame) {
		self.frames.entry(frame.number).or_insert(frame);
	}

	pub fn is_ready(&self) -> bool {
		let max = self.frames.len() as u16;
		for i in 0..max {
			if !self.frames.contains_key(&i) {
				return false;
			}
		}
		return self.frames.get(&(max - 1)).unwrap().is_last;
	}

	pub fn data(&mut self) -> Option<Vec<u8>> {
		let max = self.frames.len() as u16;
		if !self.is_ready() {
			return None;
		}
		// TODO: Check is closed
		let mut out = Vec::new();
		for i in 0..max {
			let data = &mut self.frames.get_mut(&i).unwrap().data;
			out.append(data);
		}
		Some(out)
	}
}

impl ChannelBank {
	pub fn load_frames(&mut self, frames: Vec<Frame>) {
		for frame in frames {
			if let std::collections::hash_map::Entry::Vacant(e) = self.channels_map.entry(frame.id) {
				e.insert(Channel::default());
				self.channels_by_creation.push_back(frame.id);
			}
			self.channels_map.get_mut(&frame.id).unwrap().load_frame(frame);
			// TODO: prune
		}
	}

	pub fn get_channel_data(&mut self) -> Option<Vec<u8>> {
		let curr = self.channels_by_creation.front()?;
		let ch = self.channels_map.get(curr).unwrap();

		if ch.is_ready() {
			let mut ch = self.channels_map.remove(curr).unwrap();
			self.channels_by_creation.pop_front();

			// TODO: Check if channel is timed out before returning
			return ch.data();
		}

		None
	}
}

#[derive(Default)]
pub struct BatchQueue {
	l1_blocks: VecDeque<BlockWithReceipts>, // TODO: Block ID here
	// Map batch timestamp to batches in order that they were received
	batches: HashMap<u64, VecDeque<Batch>>,
}

impl BatchQueue {
	pub fn load_batches(&mut self, batches: Vec<Batch>, _l1_origin: &Header) {
		for b in batches {
			println!("{b:?}");
		}
	}
}

fn channel_bytes_to_batches(data: Vec<u8>) -> Vec<Batch> {
	let mut decomp = ZlibDecoder::new(&data[..]);
	let mut buffer = Vec::default();

	// TODO: Handle this error
	// Decompress the passed data with zlib
	decomp.read_to_end(&mut buffer).unwrap();
	let mut buf: &[u8] = &buffer;

	// TODO: Truncate data to 10KB (post compression)
	// The data we received is an RLP encoded string. Before decoding the batch itself,
	// we need to decode the string to get the actual batch data.
	let mut decoded_batches: Vec<Vec<u8>> = Vec::new();
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

	decoded_batches.iter().map(|b| decode(b)).filter_map(|b| b.ok()).collect()
}

fn frames_from_transactions(transactions: Vec<Transaction>) -> Vec<Frame> {
	let batcher_address = Address::from_str("0x7431310e026B69BFC676C0013E12A1A11411EEc9").unwrap();

	transactions
		.iter()
		.filter(|tx| tx.from == batcher_address)
		.flat_map(|tx| parse_frames(&tx.input))
		.collect()
}

#[derive(Default)]
struct Derivation {
	channel_bank: ChannelBank,
	batch_queue: BatchQueue,
}

impl Derivation {
	pub fn load_l1_data(&mut self, header: Header, transactions: Vec<Transaction>, _receipts: Vec<TransactionReceipt>) {
		let frames = frames_from_transactions(transactions);
		self.channel_bank.load_frames(frames);
		let mut batches = Vec::new();
		while let Some(data) = self.channel_bank.get_channel_data() {
			let mut b = channel_bytes_to_batches(data);
			batches.append(&mut b);
		}
		self.batch_queue.load_batches(batches, &header);
	}

	pub fn next_l2_attributes(_l2_head: Header) -> Header {
		todo!()
	}
}

fn main() -> Result<()> {
	// Load environment variables from local ".env" file
	dotenv().ok();

	let provider = std::env::var("RPC")?;
	let mut provider = Client::new(&provider)?;
	let hash = H256::from_str("0x20ffc57ae0c607d4b612662251738b01c44f8a9a42a1da89a881a56a5fad426e")?;

	let header = provider.get_header(hash)?;
	let tx_root_hash = ethers_core::types::H256::from(header.transactions_root.as_fixed_bytes());
	let transactions = provider.get_transactions_by_root(tx_root_hash)?;
	let receipts_root_hash = ethers_core::types::H256::from(header.receipts_root.as_fixed_bytes());
	let receipts = provider.get_receipts_by_root(receipts_root_hash)?;

	let mut derivation = Derivation::default();
	derivation.load_l1_data(Header::default(), transactions, receipts);

	Ok(())
}
