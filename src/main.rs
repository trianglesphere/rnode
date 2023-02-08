use std::error::Error;

use ethers_core::{
	abi::AbiDecode,
	types::{Block, Transaction, TransactionReceipt, TxHash, H256},
};
use ethers_providers::{Http, Middleware, Provider};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
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
	pub fn new(url: &str) -> Result<Self, Box<dyn Error>> {
		let provider = Provider::<Http>::try_from(url)?;
		let rt = tokio::runtime::Builder::new_current_thread().enable_all().build()?;

		Ok(Client { rt, provider })
	}

	fn get_transaction_receipt(&self, transaction_hash: H256) -> Result<TransactionReceipt, Box<dyn Error>> {
		let receipt = self.rt.block_on(self.provider.get_transaction_receipt(transaction_hash))?;

		match receipt {
			Some(receipt) => Ok(receipt),
			None => Err("did not find the receipt".into()),
		}
	}

	pub fn get_block_with_receipts(&self, hash: H256) -> Result<BlockWithReceipts, Box<dyn Error>> {
		let block = self.rt.block_on(self.provider.get_block_with_txs(hash))?;
		let block = if let Some(b) = block {
			b
		} else {
			return Err("did not find the block".into());
		};

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

fn main() -> Result<(), Box<dyn Error>> {
	let provider = "";
	let provider = Client::new(provider)?;

	let hash = H256::decode_hex("0xee9dd94ebc06b50d5d5c0f72299a3cc56737e459ce41ddb44f0411870f86b1a3")?;

	let block = provider.get_block_with_receipts(hash)?;
	println!("Got block: {}", serde_json::to_string_pretty(&block)?);

	Ok(())
}
