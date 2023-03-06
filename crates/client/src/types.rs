use core::types::Header;
use ethers_core::types::{Block, Transaction};

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
		withdrawals_root: None,
	})
}
