use dotenv::dotenv;
use eyre::Result;

use client::prelude::*;
use derivation::derivation::Derivation;

fn main() -> Result<()> {
	// Load environment variables from local ".env" file
	dotenv().ok();

	let provider = std::env::var("RPC")?;
	let mut provider = Client::new(&provider)?;

	let mut derivation = Derivation::new(core::chain_config::GOERLI_CONFIG);
	derivation.run(8300532, 8300533, &mut provider);

	Ok(())
}
