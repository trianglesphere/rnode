use crate::types::*;

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

impl From<Header> for BlockID {
	fn from(h: Header) -> Self {
		Self {
			hash: h.hash_slow(),
			number: h.number,
			parent_hash: h.parent_hash,
		}
	}
}

impl From<L1BlockRef> for BlockID {
	fn from(h: L1BlockRef) -> Self {
		Self {
			hash: h.hash,
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
