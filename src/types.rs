use ethers_core::types::{Block, Transaction, TransactionReceipt};
pub use reth_primitives::Header;
use reth_primitives::H256;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct BlockID {
	pub hash: H256,
	pub number: u64,
	pub parent_hash: H256,
}

impl Ord for BlockID {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.number.cmp(&other.number)
	}
}

impl PartialOrd for BlockID {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.number.cmp(&other.number))
	}
}

#[derive(Debug, Clone, Copy, Default)]
pub struct L1BlockRef {
	pub hash: H256,
	pub number: u64,
	pub parent_hash: H256,
	pub time: u64,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct L2BlockRef {
	pub hash: H256,
	pub number: u64,
	pub parent_hash: H256,
	pub time: u64,
	pub l1_origin: BlockID,
	pub sequence_number: u64,
}

#[derive(Debug)]
pub struct L2BlockCandidate {
	// pub parent_hash: H256,
	pub number: u64,
	pub timestamp: u64,
	pub transactions: Vec<Transaction>,
	// TODO: tx root
}

impl From<Header> for BlockID {
	fn from(h: Header) -> Self {
		Self {
			hash: h.hash_slow(),
			number: h.number,
			parent_hash: h.parent_hash,
		}
	}
}

impl From<Header> for L1BlockRef {
	fn from(h: Header) -> Self {
		Self {
			hash: h.hash_slow(),
			number: h.number,
			parent_hash: h.parent_hash,
			time: h.timestamp,
		}
	}
}

/// A block with its receipts
#[derive(Serialize, Deserialize, Debug)]
pub struct BlockWithReceipts {
	/// The block
	pub block: Block<Transaction>,
	/// The receipts
	pub receipts: Vec<TransactionReceipt>,
}

/// Constructs a header from a given block
pub fn header_from_block(block: Block<Transaction>) -> eyre::Result<Header> {
	let author = block.author.ok_or_else(|| eyre::eyre!("block author is not set"))?;
	let number = block.number.ok_or_else(|| eyre::eyre!("block number is not set"))?;
	let bloom = block.logs_bloom.ok_or_else(|| eyre::eyre!("block logs bloom is not set"))?;
	let mix_hash = block
		.mix_hash
		.map(|h| reth_primitives::H256::from(h.as_fixed_bytes()))
		.ok_or_else(|| eyre::eyre!("block mix hash is not set"))?;
	let nonce = block.nonce.ok_or_else(|| eyre::eyre!("block nonce is not set"))?;
	let nonce = nonce.to_low_u64_be();
	Ok(Header {
		parent_hash: reth_primitives::H256::from(block.parent_hash.as_fixed_bytes()),
		ommers_hash: reth_primitives::H256::from(block.uncles_hash.as_fixed_bytes()),
		state_root: reth_primitives::H256::from(block.state_root.as_fixed_bytes()),
		beneficiary: reth_primitives::H160::from(author.as_fixed_bytes()),
		transactions_root: reth_primitives::H256::from(block.transactions_root.as_fixed_bytes()),
		receipts_root: reth_primitives::H256::from(block.receipts_root.as_fixed_bytes()),
		number: number.as_u64(),
		logs_bloom: reth_primitives::Bloom::from(bloom.as_fixed_bytes()),
		gas_used: block.gas_used.as_u64(),
		gas_limit: block.gas_limit.as_u64(),
		extra_data: reth_primitives::Bytes::from(block.extra_data.to_vec()),
		timestamp: block.timestamp.as_u64(),
		difficulty: block.difficulty.into(),
		mix_hash,
		nonce,
		base_fee_per_gas: block.base_fee_per_gas.map(|b| b.as_u64()),
	})
}
