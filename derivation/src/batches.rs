use ethers_core::{
	types::H256,
	utils::rlp::{decode, decode_list, Decodable, DecoderError, Rlp},
};
use eyre::Result;

/// A batch of transactions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatchV1 {
	/// The parent hash
	pub parent_hash: H256,
	/// The epoch number
	pub epoch_num: u64,
	/// The epoch hash
	pub epoch_hash: H256,
	/// The timestamp
	pub timestamp: u64,
	/// The transactions
	pub transactions: Vec<Vec<u8>>,
}

impl Decodable for BatchV1 {
	fn decode(rlp: &Rlp<'_>) -> Result<Self, DecoderError> {
		let parent_hash: H256 = rlp.val_at(0)?;
		let epoch_num: u64 = rlp.val_at(1)?;
		let epoch_hash: H256 = rlp.val_at(2)?;
		let timestamp: u64 = rlp.val_at(3)?;
		let transactions: Vec<Vec<u8>> = rlp.list_at(4)?;

		Ok(BatchV1 {
			parent_hash,
			epoch_num,
			epoch_hash,
			timestamp,
			transactions,
		})
	}
}

/// A batch of transactions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Batch {
	batch: BatchV1,
	// TODO: Metadata here
}

impl Decodable for Batch {
	fn decode(rlp: &Rlp<'_>) -> Result<Self, DecoderError> {
		// TODO: Make this more robust
		let first = rlp.as_raw()[0];
		if first != 0 {
			return Err(DecoderError::Custom("invalid version byte"));
		}
		let batch: BatchV1 = decode(&rlp.as_raw()[1..])?;
		Ok(Batch { batch })
	}
}

pub fn channel_bytes_to_batches(data: Vec<u8>) -> Vec<Batch> {
	// TODO: Truncate data to 10KB
	decode_list(&data)
}
