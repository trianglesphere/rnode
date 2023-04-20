use crate::frame::Frame;
use core::prelude::*;
use std::cmp::max;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Channel {
	frames: HashMap<u16, Frame>,
	id: ChannelID,
	size: u64,
	highest_frame: u16,
	end_frame: Option<u16>,
	lowest_l1_block: BlockID,
	highest_l1_block: BlockID,
}

impl Channel {
	pub fn new(id: ChannelID, l1_block: BlockID) -> Self {
		Self {
			frames: HashMap::new(),
			id,
			size: 0,
			highest_frame: 0,
			end_frame: None,
			lowest_l1_block: l1_block,
			highest_l1_block: l1_block,
		}
	}

	pub fn add_frame(&mut self, frame: Frame, l1_block: BlockID) {
		// These checks are specififed & cannot be changed without a HF
		if self.id != frame.id
			|| self.closed() && frame.is_last
			|| self.frames.contains_key(&frame.number)
			|| self.closed() && frame.number > self.highest_frame
		{
			return;
		}
		// Will always succeed at this point
		if frame.is_last {
			self.end_frame = Some(frame.number);
			// Prune higher frames if this is the closing frame
			if frame.number > self.highest_frame {
				self.frames.drain_filter(|k, _| *k > frame.number).for_each(|(_, v)| {
					self.size -= v.size();
				});
				self.highest_frame = frame.number
			}
		}

		self.highest_frame = max(self.highest_frame, frame.number);
		self.highest_l1_block = max(self.highest_l1_block, l1_block);
		self.size += frame.size();
		self.frames.insert(frame.number, frame);
	}

	pub fn is_ready(&self) -> bool {
		let last = match self.end_frame {
			Some(n) => n,
			None => return false,
		};
		(0..=last).map(|i| self.frames.contains_key(&i)).all(|a| a)
	}

	/// data returns the channel data. It will panic if `is_ready` is false.
	/// This fully consumes the channel.
	pub fn data(mut self) -> impl Iterator<Item = u8> {
		(0..=self.end_frame.unwrap()).flat_map(move |i| self.frames.remove(&i).unwrap().data)
	}

	fn closed(&self) -> bool {
		self.end_frame.is_some()
	}

	pub fn is_timed_out(&self, timeout: u64) -> bool {
		// TODO: > or >= here?
		self.highest_l1_block.number - self.lowest_l1_block.number > timeout
	}

	pub fn size(&self) -> u64 {
		self.size
	}
}
