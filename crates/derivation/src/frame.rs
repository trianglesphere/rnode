use core::types::ChannelID;
use nom::{
	branch::alt,
	bytes::complete::{tag, take},
	combinator::{map, map_res},
	multi::many0,
	number::complete::{be_u16, be_u32},
	IResult,
};

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
	parse_frames_nom(tx_data).map(|(_, frames)| frames).unwrap_or_default()
}

fn parse_frames_nom(i: &[u8]) -> IResult<&[u8], Vec<Frame>> {
	let (i, _) = tag([0])(i)?;
	let (i, frames) = many0(parse_frame)(i)?;
	Ok((i, frames))
}

fn parse_frame(i: &[u8]) -> IResult<&[u8], Frame> {
	let (i, id) = map_res(take(4usize), ChannelID::try_from)(i)?;
	let (i, number) = be_u16(i)?;
	let (i, data_len) = be_u32(i)?;
	// TODO: Validate data_len against MAX_DATA_LEN
	let (i, data) = take(data_len as usize)(i)?;
	let (i, is_last) = parse_bool(i)?;
	Ok((
		i,
		Frame {
			id,
			number,
			data: data.to_vec(),
			is_last,
		},
	))
}

fn parse_bool(i: &[u8]) -> IResult<&[u8], bool> {
	alt((map(tag([0]), |_| false), map(tag([1]), |_| true)))(i)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_bool_true() {
		assert_eq!(parse_bool(&[1]), Ok((&[][..], true)));
	}

	#[test]
	fn test_parse_bool_false() {
		assert_eq!(parse_bool(&[0]), Ok((&[][..], false)));
	}

	#[test]
	fn test_parse_bool_invalid() {
		assert!(parse_bool(&[]).is_err());
		assert!(parse_bool(&[2]).is_err());
	}
}
