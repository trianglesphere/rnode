use dotenv::dotenv;
use eyre::Result;
use hex_literal::hex;

use client::prelude::*;
use core::types::Hash;
use derivation::derivation::Derivation;

fn main() -> Result<()> {
	// Load environment variables from local ".env" file
	dotenv().ok();

	let provider = std::env::var("RPC")?;
	let mut provider = Client::new(&provider)?;
	let hash: Hash = hex!("20ffc57ae0c607d4b612662251738b01c44f8a9a42a1da89a881a56a5fad426e").into();

	let header = provider.get_header(hash)?;
	let transactions = provider.get_transactions_by_root(header.transactions_root.into())?;
	let receipts = provider.get_receipts_by_root(header.receipts_root.into())?;

	let mut derivation = Derivation::default();
	derivation.load_l1_data(header.into(), transactions, receipts);

	Ok(())
}
