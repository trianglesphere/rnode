pub type H128 = reth_primitives::H128;
pub type Address = reth_primitives::H160;
pub type H256 = reth_primitives::H256;
pub type Header = reth_primitives::Header;
pub type Receipt = ethers_core::types::TransactionReceipt;

pub fn h256_to_ethers(h: H256) -> ethers_core::types::H256 {
	ethers_core::types::H256::from_slice(h.as_bytes())
}

pub fn ethers_h256_to_h256(h: ethers_core::types::H256) -> H256 {
	H256::from_slice(h.as_bytes())
}

pub fn ethers_h160_to_h160(h: ethers_core::types::H160) -> Address {
	Address::from_slice(h.as_bytes())
}

#[derive(Debug, Clone)]
pub struct Transaction {
	pub hash: H256,
	pub from: Address,
	pub input: Vec<u8>,
}

impl From<ethers_core::types::Transaction> for Transaction {
	fn from(value: ethers_core::types::Transaction) -> Self {
		Transaction {
			hash: ethers_h256_to_h256(value.hash),
			from: ethers_h160_to_h160(value.from),
			input: value.input.to_vec(),
		}
	}
}
