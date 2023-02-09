use ethers_core::types::H256;
use ethers_providers::{Http, Provider};
use eyre::Result;
use tokio::runtime::Runtime;

/// The oracle's preimage
#[derive(Debug)]
pub struct PreImage {
	/// The hash of the preimage
	pub hash: H256,
	/// The preimage
	pub preimage: Vec<u8>,
}

/// ## Oracle
///
/// The oracle trait defines the highest level interface for a pre-image oracle.
pub trait Oracle {
	fn get_preimage(&self, hash: H256) -> Result<PreImage>;
}

/// Stateful oracle
#[derive(Debug)]
pub struct StatefulOracle {
	/// The internal web3 provider
	pub provider: Provider<Http>,
	/// The oracle runtime
	pub rt: Runtime,
}

impl Oracle for StatefulOracle {
	fn get_preimage(&self, _hash: H256) -> Result<PreImage> {
		Err(eyre::eyre!("not implemented"))
	}
}
