use core::types::ChannelID;

#[derive(Debug)]
pub struct Frame {
	pub id: ChannelID,
	pub number: u16,
	pub data: Vec<u8>,
	pub is_last: bool,
}

impl Frame {
	pub fn size(&self) -> u64 {
		self.data.len() as u64 + 200
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
		let id = ChannelID::from_slice(&tx_data[0..16]);
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
