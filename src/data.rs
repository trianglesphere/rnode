use ethers_core::{
	types::{H128, H256},
	utils::rlp::{decode, Decodable, DecoderError, Rlp},
};

// // ConfigUpdateEventABI      = "ConfigUpdate(uint256,uint8,bytes)"
// // ConfigUpdateEventABIHash  = crypto.Keccak256Hash([]byte(ConfigUpdateEventABI))
// // ConfigUpdateEventVersion0 = common.Hash{}

// struct SystemConfig {
// 	batcher_addr: Address,
// 	overhead: H256,
// 	scalar: H256,
// 	gas_limit: u64,
// }

// fn system_config_from_receipts(receipts: Vec<TransactionReceipt>, prev: SystemConfig) -> SystemConfig {
// 	let l1_system_config_addr = Address::from_str("").unwrap();
// 	let config_update_abi = H256::from_str("1d2b0bda21d56b8bd12d4f94ebacffdfb35f5e226f84b461103bb8beab6353be").unwrap();
// 	let _logs: Vec<&Log> = receipts
// 		.iter()
// 		.filter(|r| r.status == Some(1.into()))
// 		.flat_map(|r| r.logs.iter())
// 		.filter(|l| l.address == l1_system_config_addr)
// 		.filter(|l| l.topics.len() > 1 && l.topics[0] == config_update_abi)
// 		.collect();
// 	prev
// }

#[derive(Debug)]
pub struct Frame {
	pub id: H128,
	pub number: u16,
	pub data: Vec<u8>,
	pub is_last: bool,
}

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

// TODO: Clean this up
pub fn parse_frames(tx_data: &[u8]) -> Vec<Frame> {
	if tx_data.is_empty() || tx_data[0] != 0 {
		return Vec::default();
	}
	let mut tx_data = &tx_data[1..];

	let mut out = Vec::new();
	loop {
		let id = H128::from_slice(&tx_data[0..16]);
		let number = u16::from_be_bytes(tx_data[16..18].try_into().unwrap());
		let len = u32::from_be_bytes(tx_data[18..22].try_into().unwrap());
		let ulen = len as usize;
		let data = tx_data[22..22 + ulen].to_vec();
		// dbg!(id, number, len, data.len());
		let is_last = tx_data[22 + ulen];
		let is_last = if is_last == 0 {
			false
		} else if is_last == 1 {
			true
		} else {
			return Vec::default();
		};

		tx_data = &tx_data[22 + ulen + 1..];

		out.push(Frame { id, number, data, is_last });

		if tx_data.is_empty() {
			return out;
		}
	}
}
