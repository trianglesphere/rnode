use ethers_core::{
	abi::AbiDecode,
	types::{Address, Log, TransactionReceipt, H256},
};

/// ConfigUpdateEventABI is the ABI for the ConfigUpdate event.
#[allow(unused)]
pub const CONFIG_UPDATE_EVENT_ABI: &str = "ConfigUpdate(uint256,uint8,bytes)";

/// ConfigUpdateEventABIHash is the keccak256 hash of the ConfigUpdate event ABI.
// pub const ConfigUpdateEventABIHash: H256 = crypto.Keccak256Hash([]byte(ConfigUpdateEventABI));

/// ConfigUpdateEventVersion0 is the hash of the ConfigUpdate event ABI for version 0.
// pub const ConfigUpdateEventVersion0: H256 = H256::default();

/// SystemConfig is the configuration for the derivation pipeline.
#[derive(Debug, Clone, Default)]
pub struct SystemConfig {
	/// The batcher address.
	pub batcher_addr: Address,
	/// Overhead for the batcher.
	pub overhead: H256,
	/// The L1 scalar.
	pub scalar: H256,
	/// The accepted gas limit.
	pub gas_limit: u64,
}

impl From<Vec<TransactionReceipt>> for SystemConfig {
	fn from(receipts: Vec<TransactionReceipt>) -> Self {
		SystemConfig::system_config_from_receipts(receipts, Default::default())
	}
}

impl From<TransactionReceipt> for SystemConfig {
	fn from(receipt: TransactionReceipt) -> Self {
		SystemConfig::system_config_from_receipts(vec![receipt], Default::default())
	}
}

impl SystemConfig {
	/// Pushes a new set of receipts to the system config.
	pub fn push_receipts(&mut self, receipts: Vec<TransactionReceipt>) {
		// TODO: rip out pushing receipts from the [system_config_from_receipts] function
		*self = SystemConfig::system_config_from_receipts(receipts, self.clone());
	}
}

impl SystemConfig {
	pub fn system_config_from_receipts(receipts: Vec<TransactionReceipt>, prev: SystemConfig) -> SystemConfig {
		let l1_system_config_addr = Address::decode_hex("").unwrap();
		let config_update_abi = H256::decode_hex("1d2b0bda21d56b8bd12d4f94ebacffdfb35f5e226f84b461103bb8beab6353be").unwrap();
		let _logs: Vec<&Log> = receipts
			.iter()
			.filter(|r| r.status == Some(1.into()))
			.flat_map(|r| r.logs.iter())
			.filter(|l| l.address == l1_system_config_addr)
			.filter(|l| l.topics.len() > 1 && l.topics[0] == config_update_abi)
			.collect();
		prev
	}
}
