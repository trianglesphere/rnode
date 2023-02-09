use dotenv::dotenv;
use ethers_core::abi::AbiDecode;
use ethers_core::types::H256;

use client::Client;
use eyre::Result;

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
