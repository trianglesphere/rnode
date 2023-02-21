use crate::Provider;
use core::prelude::*;
use core::types::{ethers_h256_to_h256, h256_to_ethers};

use ethers_providers::{Http, Middleware, Provider as RPCProvider};
use eyre::Result;
pub use reth_primitives::Header;
use std::{collections::HashMap, convert::TryFrom};
use tokio::runtime::Runtime;

/// Client wraps a web3 provider to provide L1 pre-image oracle support.
#[derive(Debug)]
pub struct Client {
	/// The internal web3 provider
	pub provider: RPCProvider<Http>,
	/// The client runtime
	pub rt: Runtime,
	/// Store of receipts from Receipt Root to Receipts
	pub receipts: HashMap<H256, Vec<Receipt>>,
	/// Store of transactions from Transaction Root to Transactions
	pub transactions: HashMap<H256, Vec<Transaction>>,
}

impl Provider for Client {
	/// Gets a block header by block hash
	fn get_header(&mut self, hash: H256) -> Result<Header> {
		let hash = h256_to_ethers(hash);
		let block = self.rt.block_on(self.provider.get_block_with_txs(hash))?;
		let block = block.ok_or(eyre::eyre!("did not find the block"))?;

		let txs: Vec<Transaction> = block.transactions.clone().into_iter().map(|t| t.into()).collect();
		let tx_root = ethers_h256_to_h256(block.transactions_root);

		let receipts = self.get_receipts_by_transactions(&txs)?;
		let receipt_root = ethers_h256_to_h256(block.receipts_root);

		self.transactions.insert(tx_root, txs);
		self.receipts.insert(receipt_root, receipts);

		let header = crate::types::header_from_block(block)?;
		Ok(header)
	}

	/// Get receipts by the recipt root
	fn get_receipts_by_root(&self, root: H256) -> Result<Vec<Receipt>> {
		self.receipts
			.get(&root)
			.ok_or(eyre::eyre!("missing receipts for given root in internal store"))
			.cloned()
	}

	/// Get transactions by the transaction root
	fn get_transactions_by_root(&self, root: H256) -> Result<Vec<Transaction>> {
		self.transactions
			.get(&root)
			.ok_or(eyre::eyre!("missing transactions for given root in internal store"))
			.cloned()
	}
}

impl Client {
	/// Constructs a new client
	pub fn new(url: &str) -> Result<Self> {
		let provider = RPCProvider::<Http>::try_from(url)?;
		let rt = tokio::runtime::Builder::new_current_thread().enable_all().build()?;

		Ok(Client {
			rt,
			provider,
			receipts: HashMap::new(),
			transactions: HashMap::new(),
		})
	}

	/// Get transaction receipts for a list of transactions
	fn get_receipts_by_transactions(&self, transactions: &[Transaction]) -> Result<Vec<Receipt>> {
		let mut receipts = Vec::new();
		for tx in transactions.iter() {
			let receipt = self.get_transaction_receipt(tx.hash)?;
			receipts.push(receipt)
		}
		Ok(receipts)
	}

	/// Gets a transaction receipt by transaction hash
	fn get_transaction_receipt(&self, transaction_hash: H256) -> Result<Receipt> {
		let transaction_hash = h256_to_ethers(transaction_hash);
		let receipt = self.rt.block_on(self.provider.get_transaction_receipt(transaction_hash))?;
		let receipt = receipt.ok_or(eyre::eyre!("did not find the receipt"))?;
		Ok(receipt)
	}
}
