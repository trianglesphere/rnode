#![allow(dead_code)]
#![feature(hash_drain_filter)]
#![feature(is_some_and)]
#![feature(let_chains)]
#![feature(associated_type_bounds)]
#![feature(type_alias_impl_trait)]

use dotenv::dotenv;
use ethers_core::types::H256;
use eyre::Result;
use std::str::FromStr;

use src::prelude::*;

fn main() -> Result<()> {
	// Load environment variables from local ".env" file
	dotenv().ok();

	let provider = std::env::var("RPC")?;
	let mut provider = Client::new(&provider)?;
	let hash = H256::from_str("0x20ffc57ae0c607d4b612662251738b01c44f8a9a42a1da89a881a56a5fad426e")?;

	let header = provider.get_header(hash)?;
	dbg!(header);

	Ok(())
}
