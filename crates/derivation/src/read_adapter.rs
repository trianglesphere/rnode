/// ReadAdapter provides a Read method for Iterator<Item=u8> objects.
// TODO: Should I be Iterator<Item = u8>>?
pub struct ReadAdpater<I> {
	iter: I,
}

// TODO: Should I be Iterator<Item = u8>>?
impl<I> ReadAdpater<I> {
	pub fn new(iter: I) -> Self {
		Self { iter }
	}
}

impl<I: Iterator<Item = u8>> std::io::Read for ReadAdpater<I> {
	fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
		let max = buf.len();
		let mut i: usize = 0;
		while i < max && let Some(b) = self.iter.next() {
			buf[i] = b;
			i +=1;
		}
		Ok(i)
	}
}
