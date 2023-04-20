use core::types::ChannelID;
use eyre::Result;
use std::io::Read;

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

pub fn parse_frames(tx_data: &[u8]) -> Vec<Frame> {
	let mut r = tx_data;
	// Check the version byte
	let mut version_buf = [1];
	if r.read_exact(&mut version_buf).is_err() || version_buf[0] != 0 {
		return Vec::default();
	}
	// Iterate through the rest of the frames. No frames are returned if any error is encountered.
	let mut out = Vec::new();
	loop {
		if r.is_empty() {
			return out;
		}
		match parse_frame(&mut r) {
			Ok(f) => out.push(f),
			Err(_) => return Vec::default(),
		}
	}
}

fn parse_frame(r: &mut impl Read) -> Result<Frame> {
	let mut id_buf = [0u8; 16];
	let mut number_buf = [0u8; 2];
	let mut len_buf = [0u8; 4];
	let mut is_last_buf = [0u8; 1];

	r.read_exact(&mut id_buf)?;
	r.read_exact(&mut number_buf)?;
	r.read_exact(&mut len_buf)?;

	let len = u32::from_be_bytes(len_buf);
	let mut data = vec![0u8; len as usize];
	r.read_exact(&mut data)?;
	r.read_exact(&mut is_last_buf)?;

	let is_last = if is_last_buf[0] == 0 {
		false
	} else if is_last_buf[0] == 1 {
		true
	} else {
		return Err(eyre::eyre!("is_last byte is invalid"));
	};

	let id = ChannelID::new(id_buf);
	let number = u16::from_be_bytes(number_buf);

	Ok(Frame { id, number, data, is_last })
}
