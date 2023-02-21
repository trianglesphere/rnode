use super::channel::Channel;
use super::frame::Frame;
use core::prelude::*;

use core::types::H128;
use std::collections::{HashMap, VecDeque};

const MAX_CHANNEL_BANK_SIZE: u64 = 100_000_000;

#[derive(Default, Debug)]
pub struct ChannelBank {
	channels_map: HashMap<H128, Channel>,
	channels_by_creation: VecDeque<H128>,
}

impl ChannelBank {
	pub fn load_frame(&mut self, frame: Frame, l1_block: BlockID) {
		assert!(
			!self.peek().is_some_and(|c| c.is_ready()),
			"Specs Violation: must pull data before loading more in the channel bank"
		);

		self.channels_map
			.entry(frame.id)
			.or_insert_with(|| {
				self.channels_by_creation.push_back(frame.id);
				Channel::new(frame.id, l1_block)
			})
			.add_frame(frame, l1_block);
		self.prune();
	}

	pub fn get_ready_channel(&mut self) -> Option<Channel> {
		if self.peek()?.is_ready() {
			let ch = self.remove().unwrap();
			if !ch.is_timed_out() {
				return Some(ch);
			}
		}
		None
	}

	fn peek(&self) -> Option<&Channel> {
		self.channels_map.get(self.channels_by_creation.front()?)
	}

	fn remove(&mut self) -> Option<Channel> {
		self.channels_map.remove(&self.channels_by_creation.pop_front()?)
	}

	fn prune(&mut self) {
		while self.total_size() > MAX_CHANNEL_BANK_SIZE {
			self.remove().expect("Should have removed a channel");
		}
	}

	fn total_size(&self) -> u64 {
		self.channels_map.values().map(|c| c.size()).sum()
	}
}

/// ChannelBankAdapter providers an iterator for outputting ready channels.
pub struct ChannelBankAdapter<'a, I> {
	inner: I,
	cb: &'a mut ChannelBank,
	l1_block: BlockID,
}

impl<'a, I: Iterator<Item = Frame>> Iterator for ChannelBankAdapter<'a, I> {
	type Item = Channel;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			if let Some(ch) = self.cb.get_ready_channel() {
				return Some(ch);
			}
			self.cb.load_frame(self.inner.next()?, self.l1_block);
		}
	}
}

impl<'a, I> ChannelBankAdapter<'a, I> {
	pub fn new(iter: I, cb: &'a mut ChannelBank, l1_block: BlockID) -> Self {
		Self { inner: iter, cb, l1_block }
	}
}

/// ChannelBankAdapterIteratorExt allows ChannelBankAdapter to be chained onto an iterator of frames.
pub trait ChannelBankAdapterIteratorExt<'a, I>: Iterator<Item = Frame> + Sized {
	fn reassemble_channels(self, cb: &'a mut ChannelBank, l1_block: BlockID) -> ChannelBankAdapter<'a, Self> {
		ChannelBankAdapter::new(self, cb, l1_block)
	}
}

impl<'a, I: Iterator<Item = Frame>> ChannelBankAdapterIteratorExt<'a, I> for I {}
