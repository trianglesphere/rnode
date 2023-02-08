use std::error::Error;

use ethers_providers::{Http, Middleware, Provider};
use std::convert::TryFrom;

use tokio::runtime::Handle;

use ethers_core::types::{Block, TxHash};

struct Client {
	provider: Provider<Http>,
}

impl Client {
	pub fn new(url: &str) -> Result<Self, Box<dyn Error>> {
		let provider = Provider::<Http>::try_from(url)?;
		Ok(Client { provider })
	}

	pub fn get_block(&self, n: u64) -> Result<Option<Block<TxHash>>, Box<dyn Error>> {
		println!("getting block");
		let handle = Handle::current();
		// handle.enter();
		futures::executor::block_on(self.provider.get_block(n)).map_err(|e| e.into())
	}
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
	let provider = "";
	let provider = Client::new(provider)?;

	let block = provider.get_block(100u64)?;
	println!("Got block: {}", serde_json::to_string(&block)?);

	Ok(())
}
