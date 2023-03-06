use ethers_core::{
	types::H256, // Use ethers core H256 b/c it implements decodable
	utils::rlp::{decode, Decodable, DecoderError, Rlp},
};
use eyre::Result;

#[derive(Debug)]
pub struct BatchV1 {
	pub parent_hash: H256,
	pub epoch_num: u64,
	pub epoch_hash: H256,
	pub timestamp: u64,
	pub transactions: Vec<Vec<u8>>,
}

#[derive(Debug)]
pub struct Batch {
	pub batch: BatchV1,
	// TODO: Metadata here
}

impl Decodable for Batch {
	fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
		// TODO: Make this more robust
		let first = rlp.as_raw()[0];
		if first != 0 {
			return Err(DecoderError::Custom("invalid version byte"));
		}
		let batch: BatchV1 = decode(&rlp.as_raw()[1..])?;
		Ok(Batch { batch })
	}
}

impl Decodable for BatchV1 {
	fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
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
