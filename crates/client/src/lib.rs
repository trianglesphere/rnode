use core::types::{Hash, Header, Receipt, Transaction};
use eyre::Result;

pub trait Provider {
	fn get_header(&mut self, hash: Hash) -> Result<Header>;
	fn get_receipts_by_root(&self, root: Hash) -> Result<Vec<Receipt>>;
	fn get_transactions_by_root(&self, root: Hash) -> Result<Vec<Transaction>>;
}

pub mod rpc_provider;
mod types;

pub mod prelude {
	pub use crate::rpc_provider::Client;
	pub use crate::Provider;
}
