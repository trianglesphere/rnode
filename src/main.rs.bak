#![allow(dead_code)]
#![feature(hash_drain_filter)]
#![feature(is_some_and)]
#![feature(let_chains)]
#![feature(associated_type_bounds)]
#![feature(type_alias_impl_trait)]

use dotenv::dotenv;
use ethers_core::{
	types::{Address, Transaction, TransactionReceipt, H128, H256},
	utils::rlp::{decode, Rlp},
};
use eyre::Result;
use flate2::read::ZlibDecoder;
use std::cmp::max;
use std::{collections::hash_map::Entry, io::Read};
use std::{
	collections::{HashMap, VecDeque},
	str::FromStr,
};

// TODO: Should I be Iterator<Item = u8>>?
struct ReadAdpater<I> {
	inner: I,
}

// TODO: Should I be Iterator<Item = u8>>?
impl<I> ReadAdpater<I> {
	pub fn new(iter: I) -> Self {
		Self { inner: iter }
	}
}

impl<I: Iterator<Item = u8>> Read for ReadAdpater<I> {
	fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
		let max = buf.len();
		let mut i: usize = 0;
		while i < max && let Some(b) = self.inner.next() {
			buf[i] = b;
			i +=1;
		}
		Ok(i)
	}
}

/// The client module
pub mod client;
pub use client::*;

// Module Types
pub mod types;
pub use types::*;

// Module Data
pub mod data;
pub use data::*;

#[derive(Debug)]
struct Channel {
	frames: HashMap<u16, Frame>,
	id: H128,
	size: u64,
	highest_frame: u16,
	end_frame: Option<u16>,
	lowest_l1_block: BlockID,
	highest_l1_block: BlockID,
}

struct ChannelBankAdapter<'a, I> {
	inner: I,
	cb: &'a mut ChannelBank,
	l1_block: BlockID,
}

impl<'a, I: Iterator<Item = Frame>> Iterator for ChannelBankAdapter<'a, I> {
	type Item = Channel;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			if let Some(ch) = self.cb.get_ready_channel() {
				return Some(ch);
			}
			self.cb.load_frame(self.inner.next()?, self.l1_block);
		}
	}
}

impl<'a, I> ChannelBankAdapter<'a, I> {
	pub fn new(iter: I, cb: &'a mut ChannelBank, l1_block: BlockID) -> Self {
		Self { inner: iter, cb, l1_block }
	}
}

trait ChannelBankAdapterIteratorExt<'a, I>: Iterator<Item = Frame> + Sized {
	fn reassemble_channels(self, cb: &'a mut ChannelBank, l1_block: BlockID) -> ChannelBankAdapter<'a, Self> {
		ChannelBankAdapter::new(self, cb, l1_block)
	}
}

impl<'a, I: Iterator<Item = Frame>> ChannelBankAdapterIteratorExt<'a, I> for I {}

#[derive(Default)]
struct ChannelBank {
	channels_map: HashMap<H128, Channel>,
	channels_by_creation: VecDeque<H128>,
}

impl Channel {
	pub fn new(id: H128, l1_block: BlockID) -> Self {
		Self {
			frames: HashMap::new(),
			id,
			size: 0,
			highest_frame: 0,
			end_frame: None,
			lowest_l1_block: l1_block,
			highest_l1_block: l1_block,
		}
	}

	pub fn add_frame(&mut self, frame: Frame, l1_block: BlockID) {
		// These checks are specififed & cannot be changed without a HF
		if self.id != frame.id
			|| self.closed() && frame.is_last
			|| self.frames.contains_key(&frame.number)
			|| self.closed() && frame.number > self.highest_frame
		{
			return;
		}
		// Will always succeed at this point
		if frame.is_last {
			self.end_frame = Some(frame.number);
			// Prune higher frames if this is the closing frame
			if frame.number > self.highest_frame {
				self.frames.drain_filter(|k, _| *k > frame.number).for_each(|(_, v)| {
					self.size -= v.size();
				});
				self.highest_frame = frame.number
			}
		}

		self.highest_frame = max(self.highest_frame, frame.number);
		self.highest_l1_block = max(self.highest_l1_block, l1_block);
		self.size += frame.size();
		self.frames.insert(frame.number, frame);
	}

	pub fn is_ready(&self) -> bool {
		let last = match self.end_frame {
			Some(n) => n,
			None => return false,
		};
		(0..=last).map(|i| self.frames.contains_key(&i)).all(|a| a)
	}

	// data returns the channel data. It will panic if `is_ready` is false.
	// This fully consumes the channel.
	pub fn data(mut self) -> impl Iterator<Item = u8> {
		(0..=self.end_frame.unwrap()).flat_map(move |i| self.frames.remove(&i).unwrap().data)
	}

	fn closed(&self) -> bool {
		self.end_frame.is_some()
	}

	pub fn is_timed_out(&self) -> bool {
		// TODO: > or >= here?
		self.highest_l1_block.number - self.lowest_l1_block.number > CHANNEL_TIMEOUT
	}
}

const MAX_CHANNEL_BANK_SIZE: u64 = 100_000_000;
const CHANNEL_TIMEOUT: u64 = 100;

impl ChannelBank {
	fn load_frame(&mut self, frame: Frame, l1_block: BlockID) {
		assert!(
			!self.peek().is_some_and(|c| c.is_ready()),
			"Specs Violation: must pull data before loading more in the channel bank"
		);

		self.channels_map
			.entry(frame.id)
			.or_insert_with(|| {
				self.channels_by_creation.push_back(frame.id);
				Channel::new(frame.id, l1_block)
			})
			.add_frame(frame, l1_block);
		self.prune();
	}

	pub fn get_ready_channel(&mut self) -> Option<Channel> {
		if self.peek()?.is_ready() {
			let ch = self.remove().unwrap();
			if !ch.is_timed_out() {
				return Some(ch);
			}
		}
		None
	}

	fn peek(&self) -> Option<&Channel> {
		self.channels_map.get(self.channels_by_creation.front()?)
	}

	fn remove(&mut self) -> Option<Channel> {
		self.channels_map.remove(&self.channels_by_creation.pop_front()?)
	}

	fn prune(&mut self) {
		while self.total_size() > MAX_CHANNEL_BANK_SIZE {
			self.remove().expect("Should have removed a channel");
		}
	}

	fn total_size(&self) -> u64 {
		self.channels_map.values().map(|c| c.size).sum()
	}
}

#[derive(Default)]
pub struct BatchQueue {
	l1_blocks: VecDeque<L1BlockRef>,
	// Map batch timestamp to batches in order that they were received
	batches: HashMap<u64, VecDeque<Batch>>,
}

const L2_BLOCK_TIME: u64 = 2u64;
const SEQ_WINDOW_SIZE: u64 = 3600u64;

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

#[derive(Default)]
struct Derivation {
	channel_bank: ChannelBank,
	batch_queue: BatchQueue,
}

impl Derivation {
	pub fn load_l1_data(&mut self, l1_block: L1BlockRef, transactions: Vec<Transaction>, _receipts: Vec<TransactionReceipt>) {
		// TODO: Create system config from the receipts
		let batcher_address = Address::from_str("0x7431310e026B69BFC676C0013E12A1A11411EEc9").unwrap();

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
	derivation.load_l1_data(header.into(), transactions, receipts);

	Ok(())
}
