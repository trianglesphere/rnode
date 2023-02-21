use dotenv::dotenv;
use eyre::Result;
use std::str::FromStr;

use client::prelude::*;
use core::types::H256;
use derivation::derivation::Derivation;

fn main() -> Result<()> {
	// Load environment variables from local ".env" file
	dotenv().ok();

	let provider = std::env::var("RPC")?;
	let mut provider = Client::new(&provider)?;
	let hash = H256::from_str("0x20ffc57ae0c607d4b612662251738b01c44f8a9a42a1da89a881a56a5fad426e")?;

	let header = provider.get_header(hash)?;
	let transactions = provider.get_transactions_by_root(header.transactions_root)?;
	let receipts = provider.get_receipts_by_root(header.receipts_root)?;

	let mut derivation = Derivation::default();
	derivation.load_l1_data(header.into(), transactions, receipts);

	Ok(())
}
