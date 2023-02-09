use std::collections::HashMap;

use crate::frames::Frame;

/// A channel is a collection of frames that are being streamed from the L1
#[derive(Default, Debug)]
pub struct Channel {
	/// The hashmap of frames
	pub frames: HashMap<u16, Frame>,
	// TODO: Size + handling insertion of frames
}

impl Channel {
	/// Loades a frame into the channel
	pub fn load_frame(&mut self, frame: Frame) {
		self.frames.entry(frame.number).or_insert(frame);
	}

	/// Checks if the channel is ready to be read
	pub fn is_ready(&self) -> bool {
		let max = self.frames.len() as u16;
		for i in 0..max {
			if !self.frames.contains_key(&i) {
				return false;
			}
		}
		return self.frames.get(&(max - 1)).unwrap().is_last;
	}

	/// Returns the channel data
	pub fn data(&mut self) -> Option<Vec<u8>> {
		let max = self.frames.len() as u16;
		if !self.is_ready() {
			return None;
		}
		// TODO: Check is closed
		let mut out = Vec::new();
		for i in 0..max {
			let data = &mut self.frames.get_mut(&i).unwrap().data;
			out.append(data);
		}
		Some(out)
	}
}
