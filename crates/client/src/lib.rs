use core::types::{Header, Receipt, Transaction, H256};
use eyre::Result;

pub trait Provider {
	fn get_header(&mut self, hash: H256) -> Result<Header>;
	fn get_receipts_by_root(&self, root: H256) -> Result<Vec<Receipt>>;
	fn get_transactions_by_root(&self, root: H256) -> Result<Vec<Transaction>>;
}

pub mod rpc_provider;
mod types;

pub mod prelude {
	pub use crate::rpc_provider::Client;
	pub use crate::Provider;
}
