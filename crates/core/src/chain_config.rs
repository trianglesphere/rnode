use crate::{address_literal, hash_literal, id::BlockID, types::*};

pub struct SystemConfig {
	pub batcher_address: Address,
	pub overhead: Hash,
	pub scalar: Hash,
	pub gas_limit: u64,
}

pub struct RollupConfig {
	pub l1_genesis: BlockID,
	pub l2_genesis: BlockID,
	pub l2_genesis_time: u64,
	pub system_config: SystemConfig,
	pub l2_block_time: u64,
	pub max_sequencer_drift: u64,
	pub seq_window_size: u64,
	pub channel_timeout: u64,
	pub l1_chain_id: u64,
	pub l2_chain_id: u64,
	pub batch_inbox_address: Address,
	pub deposit_contract_address: Address,
	pub l1_system_config_addres: Address,
	pub regolith_time: Option<u64>,
}

pub const GOERLI_CONFIG: RollupConfig = RollupConfig {
	l1_genesis: BlockID {
		hash: hash_literal!("6ffc1bf3754c01f6bb9fe057c1578b87a8571ce2e9be5ca14bace6eccfd336c7"),
		number: 8300214,
	},
	l2_genesis: BlockID {
		hash: hash_literal!("0f783549ea4313b784eadd9b8e8a69913b368b7366363ea814d7707ac505175f"),
		number: 4061224,
	},
	l2_genesis_time: 1673550516,
	system_config: SystemConfig {
		batcher_address: address_literal!("7431310e026B69BFC676C0013E12A1A11411EEc9"),
		overhead: hash_literal!("0000000000000000000000000000000000000000000000000000000000000834"),
		scalar: hash_literal!("00000000000000000000000000000000000000000000000000000000000f4240"),
		gas_limit: 25_000_000,
	},
	l2_block_time: 2,
	max_sequencer_drift: 600,
	seq_window_size: 3600,
	channel_timeout: 300,
	l1_chain_id: 5,
	l2_chain_id: 420,
	batch_inbox_address: address_literal!("ff00000000000000000000000000000000000420"),
	deposit_contract_address: address_literal!("5b47E1A08Ea6d985D6649300584e6722Ec4B1383"),
	l1_system_config_addres: address_literal!("Ae851f927Ee40dE99aaBb7461C00f9622ab91d60"),
	regolith_time: Some(1679079600),
};
