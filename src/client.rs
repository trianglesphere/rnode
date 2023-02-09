use ethers_core::types::{Block, Transaction, TransactionReceipt, H256};
use ethers_providers::{Http, Middleware, Provider};
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use tokio::runtime::Runtime;

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockWithReceipts {
	pub block: Block<Transaction>,
	pub receipts: Vec<TransactionReceipt>,
}

/// Client wraps a web3 provider to provide L1 pre-image oracle support.
pub struct Client {
	pub provider: Provider<Http>,
	pub rt: Runtime,
}

impl Client {
	pub fn new(url: &str) -> Result<Self> {
		let provider = Provider::<Http>::try_from(url)?;
		let rt = tokio::runtime::Builder::new_current_thread().enable_all().build()?;

		Ok(Client { rt, provider })
	}

	pub fn get_transaction_receipt(&self, transaction_hash: H256) -> Result<TransactionReceipt> {
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
