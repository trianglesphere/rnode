#![allow(dead_code)]

use dotenv::dotenv;
use ethers_core::{
	types::{Address, Log, Transaction, TransactionReceipt, H128, H256},
	utils::rlp::{decode, Decodable, DecoderError, Rlp},
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

// ConfigUpdateEventABI      = "ConfigUpdate(uint256,uint8,bytes)"
// ConfigUpdateEventABIHash  = crypto.Keccak256Hash([]byte(ConfigUpdateEventABI))
// ConfigUpdateEventVersion0 = common.Hash{}

struct SystemConfig {
	batcher_addr: Address,
	overhead: H256,
	scalar: H256,
	gas_limit: u64,
}

fn system_config_from_receipts(receipts: Vec<TransactionReceipt>, prev: SystemConfig) -> SystemConfig {
	let l1_system_config_addr = Address::from_str("").unwrap();
	let config_update_abi = H256::from_str("1d2b0bda21d56b8bd12d4f94ebacffdfb35f5e226f84b461103bb8beab6353be").unwrap();
	let _logs: Vec<&Log> = receipts
		.iter()
		.filter(|r| r.status == Some(1.into()))
		.flat_map(|r| r.logs.iter())
		.filter(|l| l.address == l1_system_config_addr)
		.filter(|l| l.topics.len() > 1 && l.topics[0] == config_update_abi)
		.collect();
	prev
}

#[derive(Debug)]
struct Frame {
	id: H128,
	number: u16,
	data: Vec<u8>,
	is_last: bool,
}

fn parse_frames(tx_data: &[u8]) -> Vec<Frame> {
	if tx_data.is_empty() && tx_data[0] != 0 {
		return Vec::default();
	}
	let mut tx_data = &tx_data[1..];

	let mut out = Vec::new();
	loop {
		let id = H128::from_slice(&tx_data[0..16]);
		let number = u16::from_be_bytes(tx_data[16..18].try_into().unwrap());
		let len = u32::from_be_bytes(tx_data[18..22].try_into().unwrap());
		let ulen = len as usize;
		let data = tx_data[22..22 + ulen].to_vec();
		// dbg!(id, number, len, data.len());
		let is_last = tx_data[22 + ulen];
		let is_last = if is_last == 0 {
			false
		} else if is_last == 1 {
			true
		} else {
			return Vec::default();
		};

		tx_data = &tx_data[22 + ulen + 1..];

		out.push(Frame { id, number, data, is_last });

		if tx_data.is_empty() {
			return out;
		}
	}
}

#[derive(Default)]
struct ChannelBank {
	channels_map: HashMap<H128, Channel>,
	channels_by_creation: VecDeque<H128>,
	// TODO: Pruning
}

#[derive(Default)]
struct Channel {
	frames: HashMap<u16, Frame>,
	// TODO: Size + handling insertion of frames
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

#[derive(Debug)]
struct BatchV1 {
	parent_hash: H256,
	epoch_num: u64,
	epoch_hash: H256,
	timestamp: u64,
	transactions: Vec<Vec<u8>>,
}

impl Decodable for BatchV1 {
	fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
		let parent_hash: H256 = rlp.val_at(0)?;
		let epoch_num: u64 = rlp.val_at(1)?;
		let epoch_hash: H256 = rlp.val_at(2)?;
		let timestamp: u64 = rlp.val_at(3)?;
		let transactions: Vec<Vec<u8>> = rlp.list_at(4)?;

		Ok(BatchV1 {
			parent_hash,
			epoch_num,
			epoch_hash,
			timestamp,
			transactions,
		})
	}
}

#[derive(Debug)]
struct Batch {
	batch: BatchV1,
	// TODO: Metadata here
}

impl Decodable for Batch {
	fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
		// TODO: Make this more robust
		let first = rlp.as_raw()[0];
		if first != 0 {
			return Err(DecoderError::Custom("invalid version byte"));
		}
		let batch: BatchV1 = decode(&rlp.as_raw()[1..])?;
		Ok(Batch { batch })
	}
}

fn channel_bytes_to_batches(data: Vec<u8>) -> Vec<Batch> {
	let mut decomp = ZlibDecoder::new(&data[..]);
	let mut buffer = Vec::default();

	// TODO: Handle this error
	// Decompress the passed data with zlib
	decomp.read_to_end(&mut buffer).unwrap();

	// TODO: Truncate data to 10KB (post compression)
	// The data we received is an RLP encoded string. Before decoding the batch itself,
	// we need to decode the string to get the actual batch data.
	let decoded_batch = decode::<Vec<u8>>(&buffer).unwrap();
	// Decode the batch itself.
	let b: Batch = decode(&decoded_batch).unwrap();

	dbg!(&b);

	vec![b]
}

fn frames_from_transactions(transactions: Vec<Transaction>) -> Vec<Frame> {
	let batcher_address = Address::from_str("0x7431310e026B69BFC676C0013E12A1A11411EEc9").unwrap();

	transactions
		.iter()
		.filter(|tx| tx.from == batcher_address)
		.flat_map(|tx| parse_frames(&tx.input))
		.collect()
}

fn main() -> Result<()> {
	// Load environment variables from local ".env" file
	dotenv().ok();

	let provider = std::env::var("RPC")?;
	let mut provider = Client::new(&provider)?;

	let hash = H256::from_str("0x20ffc57ae0c607d4b612662251738b01c44f8a9a42a1da89a881a56a5fad426e")?;

	let block = provider.get_block_with_receipts(hash)?;
	// println!("Got block: {}", serde_json::to_string_pretty(&block)?);

	let frames = frames_from_transactions(block.block.transactions);
	let mut cb = ChannelBank::default();
	cb.load_frames(frames);
	let channel_data = cb.get_channel_data();

	if let Some(d) = channel_data {
		let batches = channel_bytes_to_batches(d);
		println!("Got batches: {batches:?}");
	} else {
		println!("Invalid batch")
	}

	Ok(())
}
