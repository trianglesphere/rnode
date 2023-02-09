use ethers_core::types::H128;
use std::collections::{HashMap, VecDeque};

use crate::channels::Channel;
use crate::frames::Frame;

/// Channel bank is a collection of channels
#[derive(Default, Debug)]
pub struct ChannelBank {
	/// A map of channels
	pub channels_map: HashMap<H128, Channel>,
	/// A queue of channels by creation
	pub channels_by_creation: VecDeque<H128>,
	// TODO: Pruning
}

impl ChannelBank {
	/// Loads frames into the channel
	pub fn load_frames(&mut self, frames: Vec<Frame>) {
		for frame in frames {
			if let std::collections::hash_map::Entry::Vacant(e) = self.channels_map.entry(frame.id) {
				e.insert(Channel::default());
				self.channels_by_creation.push_back(frame.id);
			}
			self.channels_map.get_mut(&frame.id).unwrap().load_frame(frame);
			// TODO: prune
		}
	}

	/// Gets the channel data
	pub fn get_channel_data(&mut self) -> Option<Vec<u8>> {
		let curr = self.channels_by_creation.front()?;
		let ch = self.channels_map.get(curr).unwrap();

		if ch.is_ready() {
			let mut ch = self.channels_map.remove(curr).unwrap();
			self.channels_by_creation.pop_front();

			// TODO: Check if channel is timed out before returning
			return ch.data();
		}

		None
	}
}
