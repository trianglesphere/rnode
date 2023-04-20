#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct ChannelID([u8; 16]);

impl ChannelID {
	pub fn from_slice(data: &[u8]) -> Self {
		Self(data.try_into().unwrap())
	}
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Address([u8; 20]);

impl Address {
	pub fn new(v: [u8; 20]) -> Self {
		Self(v)
	}
}

impl From<reth_primitives::H160> for Address {
	fn from(value: reth_primitives::H160) -> Self {
		Self(value.to_fixed_bytes())
	}
}

impl From<ethers_core::types::H160> for Address {
	fn from(value: ethers_core::types::H160) -> Self {
		Self(value.to_fixed_bytes())
	}
}

#[macro_export]
macro_rules! address_literal {
	($s:literal) => {
		Address::new(hex_literal::hex!($s))
	};
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Hash([u8; 32]);

impl Hash {
	pub fn new(v: [u8; 32]) -> Self {
		Self(v)
	}
	pub fn to_vec(self) -> Vec<u8> {
		Vec::from(self.0)
	}
}

impl From<reth_primitives::H256> for Hash {
	fn from(value: reth_primitives::H256) -> Self {
		Self(value.to_fixed_bytes())
	}
}

impl From<ethers_core::types::H256> for Hash {
	fn from(value: ethers_core::types::H256) -> Self {
		Self(value.to_fixed_bytes())
	}
}

impl From<Hash> for ethers_core::types::H256 {
	fn from(val: Hash) -> Self {
		ethers_core::types::H256::from(val.0)
	}
}

#[macro_export]
macro_rules! hash_literal {
	($s:literal) => {
		Hash::new(hex_literal::hex!($s))
	};
}

pub type Header = reth_primitives::Header;
pub type Receipt = ethers_core::types::TransactionReceipt;

#[derive(Debug, Clone)]
pub struct Transaction {
	pub hash: Hash,
	pub from: Address,
	pub input: Vec<u8>,
}

impl From<ethers_core::types::Transaction> for Transaction {
	fn from(value: ethers_core::types::Transaction) -> Self {
		Transaction {
			hash: value.hash.into(),
			from: value.from.into(),
			input: value.input.to_vec(),
		}
	}
}

pub fn keccak(data: impl AsRef<[u8]>) -> Hash {
	reth_primitives::keccak256(data).into()
}
