use crate::Provider;
use core::prelude::*;
use core::types::{Hash, Header};

use ethers_providers::{Http, Middleware, Provider as RPCProvider};
use eyre::Result;
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
	pub receipts: HashMap<Hash, Vec<Receipt>>,
	/// Store of transactions from Transaction Root to Transactions
	pub transactions: HashMap<Hash, Vec<Transaction>>,
}

impl Provider for Client {
	/// Gets a block header by block hash
	fn get_header(&mut self, hash: Hash) -> Result<Header> {
		let hash: ethers_core::types::H256 = hash.into();
		let block = self.rt.block_on(self.provider.get_block_with_txs(hash))?;
		let block = block.ok_or(eyre::eyre!("did not find the block"))?;

		let txs: Vec<Transaction> = block.transactions.clone().into_iter().map(|t| t.into()).collect();
		let tx_root = block.transactions_root.into();

		// let receipts = self.get_receipts_by_transactions(&txs)?;
		// let receipt_root = block.receipts_root.into();

		self.transactions.insert(tx_root, txs);
		// self.receipts.insert(receipt_root, receipts);

		let header = crate::types::header_from_block(block)?;
		Ok(header)
	}

	/// Gets a block header by block number
	fn get_header_by_number(&mut self, n: u64) -> Result<Header> {
		let block = self.rt.block_on(self.provider.get_block_with_txs(n))?;
		let block = block.ok_or(eyre::eyre!("did not find the block"))?;

		let txs: Vec<Transaction> = block.transactions.clone().into_iter().map(|t| t.into()).collect();
		let tx_root = block.transactions_root.into();

		// let receipts = self.get_receipts_by_transactions(&txs)?;
		// let receipt_root = block.receipts_root.into();

		self.transactions.insert(tx_root, txs);
		// self.receipts.insert(receipt_root, receipts);

		let header = crate::types::header_from_block(block)?;
		Ok(header)
	}

	/// Get receipts by the recipt root
	fn get_receipts_by_root(&self, root: Hash) -> Result<Vec<Receipt>> {
		self.receipts
			.get(&root)
			.ok_or(eyre::eyre!("missing receipts for given root in internal store"))
			.cloned()
	}

	/// Get transactions by the transaction root
	fn get_transactions_by_root(&self, root: Hash) -> Result<Vec<Transaction>> {
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

	// /// Get transaction receipts for a list of transactions
	// fn get_receipts_by_transactions(&self, transactions: &[Transaction]) -> Result<Vec<Receipt>> {
	// 	let mut receipts = Vec::anew();
	// 	for tx in transactions.iter() {
	// 		let receipt = self.get_transaction_receipt(tx.hash)?;
	// 		receipts.push(receipt)
	// 	}
	// 	Ok(receipts)
	// }

	// /// Gets a transaction receipt by transaction hash
	// fn_get_transaction_receipt(&self, transaction_hash: Hash) -> Result<Receipt> {
	// 	let transaction_hash: ethers_core::types::H256 = transaction_hash.into();
	// 	let receipt = self.rt.block_on(self.provider.get_transaction_receipt(transaction_hash))?;
	// 	let receipt = receipt.ok_or(eyre::eyre!("did not find the receipt"))?;
	// 	Ok(receipt)
	// }
}
