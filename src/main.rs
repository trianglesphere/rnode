#![allow(dead_code)]

use dotenv::dotenv;
use ethers_core::{
	abi::AbiDecode,
	types::{Address, Block, Log, Transaction, TransactionReceipt, H128, H256},
	utils::rlp::{decode, decode_list, Decodable, DecoderError, Rlp},
};
use ethers_providers::{Http, Middleware, Provider};
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::{
	collections::{HashMap, VecDeque},
	convert::TryFrom,
};
use tokio::runtime::Runtime;

struct Client {
	provider: Provider<Http>,
	rt: Runtime,
}

#[derive(Serialize, Deserialize, Debug)]
struct BlockWithReceipts {
	block: Block<Transaction>,
	receipts: Vec<TransactionReceipt>,
}

impl Client {
	pub fn new(url: &str) -> Result<Self> {
		let provider = Provider::<Http>::try_from(url)?;
		let rt = tokio::runtime::Builder::new_current_thread().enable_all().build()?;

		Ok(Client { rt, provider })
	}

	fn get_transaction_receipt(&self, transaction_hash: H256) -> Result<TransactionReceipt> {
		let receipt = self.rt.block_on(self.provider.get_transaction_receipt(transaction_hash))?;

		receipt.ok_or(eyre::eyre!("did not find the receipt"))
	}

	pub fn get_block_with_receipts(&self, hash: H256) -> Result<BlockWithReceipts> {
		let block =
			self.rt.block_on(self.provider.get_block_with_txs(hash))?
				.ok_or(eyre::eyre!("did not find the block"))?;

		let mut receipts = Vec::new();

		for tx in block.transactions.iter() {
			let receipt = self.get_transaction_receipt(tx.hash)?;
			receipts.push(receipt)
		}

		Ok(BlockWithReceipts { block, receipts })
	}

	// pub fn get_head_block(&self) -> Result<Block<TxHash>, Box<dyn Error>> {
	// 	self.provider.get_block(block_hash_or_number)
	// }
}

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
	let l1_system_config_addr = Address::decode_hex("").unwrap();
	let config_update_abi = H256::decode_hex("1d2b0bda21d56b8bd12d4f94ebacffdfb35f5e226f84b461103bb8beab6353be").unwrap();
	let _logs: Vec<&Log> = receipts
		.iter()
		.filter(|r| r.status == Some(1.into()))
		.flat_map(|r| r.logs.iter())
		.filter(|l| l.address == l1_system_config_addr)
		.filter(|l| l.topics.len() > 1 && l.topics[0] == config_update_abi)
		.collect();
	return prev;
}

struct Frame {
	id: H128,
	number: u16,
	data: Vec<u8>,
	is_last: bool,
}

fn parse_frames(tx_data: &[u8]) -> Vec<Frame> {
	if tx_data.len() == 0 {
		return Vec::default();
	} else if tx_data[0] != 0 {
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

		if tx_data.len() == 0 {
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
		if !self.frames.contains_key(&frame.number) {
			self.frames.insert(frame.number, frame);
		}
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
			if !self.channels_map.contains_key(&frame.id) {
				self.channels_map.insert(frame.id, Channel::default());
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
	// TODO: Truncate data to 10KB
	decode_list(&data)
}

fn frames_from_transactions(transactions: Vec<Transaction>) -> Vec<Frame> {
	let batcher_address = Address::decode_hex("").unwrap();

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
	let provider = Client::new(&provider)?;

	let hash = H256::decode_hex("0xee9dd94ebc06b50d5d5c0f72299a3cc56737e459ce41ddb44f0411870f86b1a3")?;

	let block = provider.get_block_with_receipts(hash)?;
	println!("Got block: {}", serde_json::to_string_pretty(&block)?);

	Ok(())
}
