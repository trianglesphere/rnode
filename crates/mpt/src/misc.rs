use core::types::H256;
use reth_primitives::{keccak256, Bytes};
use reth_rlp::Encodable;
use std::{collections::HashMap, fmt::Debug, iter::zip};

#[derive(Debug)]
pub enum RLPEncodeableWrapper {
	EmptyString,
	Bytes(Vec<u8>),
	Raw(Vec<u8>),
}

impl Encodable for RLPEncodeableWrapper {
	fn encode(&self, out: &mut dyn reth_rlp::BufMut) {
		match self {
			Self::EmptyString => out.put_u8(0x80),
			Self::Bytes(value) => out.put_slice(&encode_bytes(value.clone())),
			Self::Raw(value) => out.put_slice(value),
		}
	}
}

// encode_bytes encodes a vec<u8> as a byte string instead of a list of u8s.
pub fn encode_bytes(x: Vec<u8>) -> Vec<u8> {
	let mut out = Vec::new();
	let b: Bytes = x.into();
	b.encode(&mut out);
	out
}

// mpt_hash implements H(x) as used in the MPT.
pub fn mpt_hash(x: &[u8], db: &mut HashMap<H256, Vec<u8>>) -> RLPEncodeableWrapper {
	if x.len() < 32 {
		RLPEncodeableWrapper::Raw(x.to_vec())
	} else {
		let h = keccak256(x);
		db.insert(h, x.to_vec());
		RLPEncodeableWrapper::Bytes(h.to_vec())
	}
}

// match_paths is a helper function that returns the shared bytes between a & b as well
// the remaining bytes in a & b.
pub fn match_paths<'a, 'b>(key: &'a [u8], path: &'b [u8]) -> (Vec<u8>, &'a [u8], &'b [u8]) {
	let mut common = Vec::new();
	for (a, b) in zip(key, path) {
		if a == b {
			common.push(*a)
		} else {
			break;
		}
	}
	let i = common.len();
	(common, &key[i..], &path[i..])
}

// bytes_to_nibbles splits a list of bytes into a list of nibbles
pub fn bytes_to_nibbles(key: &[u8]) -> Vec<u8> {
	let mut out = Vec::new();
	for byte in key {
		out.push(byte >> 4);
		out.push(byte & 0x0f);
	}
	out
}

// nibbles_to_compact turns a list of nibbles into Ethereum's compact encoding scheme.
// It prefixes the parity of the nibbles length & if it's an extension into the first nibble
// and then folds the nibbles into bytes.
pub fn nibbles_to_compact(nibbles: &[u8], extension: bool) -> Vec<u8> {
	let mut key = nibbles;
	let mut out = Vec::new();
	let even = key.len() % 2 == 0;
	let mut first = match (extension, even) {
		(true, true) => 0,
		(true, false) => 1,
		(false, true) => 2,
		(false, false) => 3,
	} << 4;
	if !key.is_empty() && !even {
		first |= key[0];
		key = &key[1..];
	}
	out.push(first);
	for a in key.chunks_exact(2) {
		out.push(a[0] << 4 | a[1]);
	}
	out
}

// compact_to_nibbles decodes Ethereum's compact encoding into the original nibbles
// array and also returns if the path was an extension or not.
#[allow(dead_code)]
pub fn compact_to_nibbles(compact: &[u8]) -> (Vec<u8>, bool) {
	let (extension, even) = match compact[0] >> 4 {
		0 => (true, true),
		1 => (true, false),
		2 => (false, true),
		3 => (false, false),
		_ => panic!("out of range"),
	};
	let mut nibbles = Vec::new();
	if !even {
		nibbles.push(compact[0] & 0x0f);
	}
	for b in &compact[1..] {
		nibbles.push(b >> 4);
		nibbles.push(b & 0x0f);
	}
	(nibbles, extension)
}
