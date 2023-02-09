use ethers_core::types::{Address, Block, Bloom, Bytes, Transaction, TransactionReceipt, H256, H64, U256, U64};

// use reth_rlp::*;
pub use reth_primitives::Header;
use serde::{Deserialize, Serialize};

/// A block with its receipts
#[derive(Serialize, Deserialize, Debug)]
pub struct BlockWithReceipts {
	/// The block
	pub block: Block<Transaction>,
	/// The receipts
	pub receipts: Vec<TransactionReceipt>,
}

/// CustomHeader represents a block header in the Ethereum blockchain.
#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct CustomHeader {
	/// Hash of the parent
	#[serde(default, rename = "parentHash")]
	pub parent_hash: H256,
	/// Hash of the uncles
	#[serde(default, rename = "sha3Uncles")]
	pub uncles_hash: H256,
	/// State root hash
	#[serde(default, rename = "stateRoot")]
	pub state_root: H256,
	/// Miner/author's address. None if pending.
	#[serde(default, rename = "miner")]
	pub author: Option<Address>,
	/// Transactions root hash
	#[serde(default, rename = "transactionsRoot")]
	pub transactions_root: H256,
	/// Transactions receipts root hash
	#[serde(default, rename = "receiptsRoot")]
	pub receipts_root: H256,
	/// Block number. None if pending.
	pub number: Option<U64>,
	/// Logs bloom
	#[serde(rename = "logsBloom")]
	pub logs_bloom: Option<Bloom>,
	/// Gas Used
	#[serde(default, rename = "gasUsed")]
	pub gas_used: U256,
	/// Gas Limit
	#[serde(default, rename = "gasLimit")]
	pub gas_limit: U256,
	/// Extra data
	#[serde(default, rename = "extraData")]
	pub extra_data: Bytes,
	/// Timestamp
	#[serde(default)]
	pub timestamp: U256,
	/// Difficulty
	#[serde(default)]
	pub difficulty: U256,
	/// Mix Hash
	#[serde(rename = "mixHash")]
	pub mix_hash: Option<H256>,
	/// Nonce
	pub nonce: Option<H64>,
	/// Base fee per unit of gas (if past London)
	#[serde(rename = "baseFeePerGas")]
	pub base_fee_per_gas: Option<U256>,
	/// Withdrawals Root
	#[serde(rename = "withdrawalsRoot")]
	pub withdrawals_root: Option<H256>,
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

// impl From<Block<Transaction>> for Header {
// 	fn from(block: Block<Transaction>) -> Self {
// 		Self {
// 			parent_hash: block.parent_hash,
// 			uncles_hash: block.uncles_hash,
// 			state_root: block.state_root,
// 			author: block.author,
// 			transactions_root: block.transactions_root,
// 			receipts_root: block.receipts_root,
// 			number: block.number,
// 			logs_bloom: block.logs_bloom,
// 			gas_used: block.gas_used,
// 			gas_limit: block.gas_limit,
// 			extra_data: block.extra_data,
// 			timestamp: block.timestamp,
// 			difficulty: block.difficulty,
// 			mix_hash: block.mix_hash,
// 			nonce: block.nonce,
// 			base_fee_per_gas: block.base_fee_per_gas,
// 			withdrawals_root: None,
// 		}
// 	}
// }
