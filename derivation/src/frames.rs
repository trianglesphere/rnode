use ethers_core::{
	abi::AbiDecode,
	types::{Address, Transaction, H128},
};

/// Derivation Frame
#[derive(Debug, Clone, Default)]
pub struct Frame {
	/// The frame id
	pub id: H128,
	/// The frame number
	pub number: u16,
	/// The frame data
	pub data: Vec<u8>,
	/// Whether this is the last frame
	pub is_last: bool,
}

impl Frame {
	/// Parses frames from a list of transactions
	pub fn frames_from_transactions(transactions: Vec<Transaction>) -> Vec<Frame> {
		let batcher_address = Address::decode_hex("").unwrap();

		transactions
			.iter()
			.filter(|tx| tx.from == batcher_address)
			.flat_map(|tx| parse_frames(&tx.input))
			.collect()
	}
}

fn parse_frames(tx_data: &[u8]) -> Vec<Frame> {
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
