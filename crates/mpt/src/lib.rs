use core::types::H256;
use reth_primitives::{keccak256, Bytes};
use reth_rlp::Encodable;
use std::{collections::HashMap, fmt::Debug, iter::zip};

#[cfg(test)]
mod mpt_test;

pub struct MPT {
	root: Node,
	db: HashMap<H256, Vec<u8>>,
}

impl MPT {
	pub fn new() -> Self {
		MPT {
			root: Node::Empty,
			db: HashMap::default(),
		}
	}
	pub fn hash(&mut self) -> H256 {
		let root = self.root.rlp_bytes(&mut self.db);
		println!("{}", hex::encode(&root));
		if root.len() < 32 {
			keccak256(root)
		} else {
			H256::from_slice(&root[..32])
		}
	}

	pub fn insert(&mut self, k: Vec<u8>, v: Vec<u8>) {
		let k = bytes_to_nibbles(&k);
		let root = std::mem::take(&mut self.root);
		self.root = root.insert(&k, v);
	}
}

impl Debug for MPT {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("root: {:#?}\n", &self.root))?;
		for (k, v) in self.db.iter() {
			f.write_fmt(format_args!("{k:?}\t0x{}\n", hex::encode(v)))?;
		}
		Ok(())
	}
}

#[derive(Debug)]
enum Node {
	Empty,
	Branch(BranchNode),
	Extension(ExtensionNode),
	Value(ValueNode),
}

impl Node {
	fn new(nibbles: &[u8], value: Vec<u8>) -> Self {
		if nibbles.is_empty() {
			ValueNode::new(value).into()
		} else {
			ExtensionNode::new(nibbles.to_owned(), ValueNode::new(value).into()).into()
		}
	}

	fn new_with_node(nibbles: &[u8], child: Box<Node>) -> Box<Self> {
		if nibbles.is_empty() {
			child
		} else {
			Box::new(ExtensionNode::new(nibbles.to_owned(), *child).into())
		}
	}

	fn insert(self, nibbles: &[u8], value: Vec<u8>) -> Self {
		match self {
			Node::Empty => Node::new(nibbles, value),
			Node::Branch(node) => node.insert(nibbles, value).unwrap().into(),
			Node::Extension(node) => {
				let (common, new_nibbles, old_nibbles) = match_paths(nibbles, &node.nibbles);
				if new_nibbles.is_empty() && old_nibbles.is_empty() {
					panic!("Paths cannot be the same");
				}
				// Inserting here will alwasy create branch node.
				// Turn the existing node into that branch node then insert the new value.
				let branch_node = if old_nibbles.is_empty() {
					match *node.child {
						Node::Empty => panic!("Cannot point to an empty node in an extension"),
						Node::Extension(..) => panic!("Cannot point to an extension node in an extension node"),
						Node::Value(child) => BranchNode::new_with_value(child),
						Node::Branch(child) => child,
					}
				} else {
					let child = Node::new_with_node(&old_nibbles[1..], node.child);
					BranchNode::new_with_node(old_nibbles[0], child)
				}
				.insert(new_nibbles, value)
				.unwrap();
				// Create an extension node based on the common part if needed.
				if common.is_empty() {
					branch_node.into()
				} else {
					ExtensionNode::new(common, branch_node.into()).into()
				}
			}
			Node::Value(..) => panic!("Cannot insert into a value node"),
		}
	}

	fn rlp_bytes(&mut self, db: &mut HashMap<H256, Vec<u8>>) -> Vec<u8> {
		match self {
			Node::Empty => vec![0x80],
			Node::Branch(node) => node.rlp_bytes(db),
			Node::Extension(node) => node.rlp_bytes(db),
			Node::Value(node) => node.rlp_bytes(db),
		}
	}
}

impl Default for Node {
	fn default() -> Self {
		Self::Empty
	}
}

struct ValueNode {
	value: Vec<u8>,
}

impl ValueNode {
	fn new(value: Vec<u8>) -> Self {
		Self { value }
	}
	fn rlp_bytes(&self, db: &mut HashMap<H256, Vec<u8>>) -> Vec<u8> {
		let h = mpt_hash(&self.value, db);
		let mut out = Vec::new();
		Bytes::from(h).encode(&mut out);
		out
	}
}

impl From<Vec<u8>> for ValueNode {
	fn from(value: Vec<u8>) -> Self {
		ValueNode::new(value)
	}
}

impl From<ValueNode> for Node {
	fn from(value: ValueNode) -> Self {
		Node::Value(value)
	}
}

impl Debug for ValueNode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{:x?}", &self.value))?;
		Ok(())
	}
}

struct ExtensionNode {
	nibbles: Vec<u8>,
	child: Box<Node>,
}

impl ExtensionNode {
	fn new(nibbles: Vec<u8>, child: Node) -> Self {
		Self {
			nibbles,
			child: Box::new(child),
		}
	}

	fn compact(&self) -> Vec<u8> {
		let extension = match *self.child {
			Node::Empty => panic!("Cannot point to an empty node in an extension"),
			Node::Extension(..) => panic!("Cannot point to an extension node in an extension node"),
			Node::Value(..) => false,
			Node::Branch(..) => true,
		};
		nibbles_to_compact(&self.nibbles, extension)
	}

	fn rlp_bytes(&mut self, db: &mut HashMap<H256, Vec<u8>>) -> Vec<u8> {
		let mut list: Vec<RLPEncodeableWrapper> = Vec::new();
		let mut bytes = Vec::new();

		list.push(RLPEncodeableWrapper::String(self.compact()));
		list.push(RLPEncodeableWrapper::Raw(self.child.rlp_bytes(db)));
		reth_rlp::encode_list(&list, &mut bytes);

		let hash = mpt_hash(&bytes, db);
		println!("{hash:?}: {:x?}", bytes);
		hash
	}
}

impl From<ExtensionNode> for Node {
	fn from(value: ExtensionNode) -> Self {
		Node::Extension(value)
	}
}

impl Debug for ExtensionNode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("nibbles: {:x?}\n", &self.nibbles))?;
		f.write_fmt(format_args!("compact: {:x?}\n", self.compact()))?;
		f.write_fmt(format_args!("child: {:#?}", self.child))?;
		Ok(())
	}
}

#[derive(Default)]
struct BranchNode {
	children: [Box<Node>; 16],
	branch_value: Option<ValueNode>,
}

impl BranchNode {
	// inserts adds a key/value to a branch node as either a sub-node or as a value.
	// It returns none if there is an error.
	pub fn insert(mut self, nibbles: &[u8], value: Vec<u8>) -> Option<Self> {
		if nibbles.is_empty() {
			if self.branch_value.is_some() {
				// TODO: Error: Cannot double insert into a branch node.
				return None;
			}
			self.branch_value = Some(value.into());
		} else {
			let i = nibbles[0] as usize;
			*self.children[i] = std::mem::take(&mut self.children[i]).insert(&nibbles[1..], value);
		};
		Some(self)
	}

	// new_with_node creates a new branch node that contains a given child node.
	pub fn new_with_node(key: u8, node: Box<Node>) -> Self {
		let mut branch_node = BranchNode::default();
		branch_node.children[key as usize] = node;
		branch_node
	}

	// new_with_value creates a new branch node that contains the given value.
	pub fn new_with_value(value: ValueNode) -> Self {
		let mut branch_node = BranchNode::default();
		branch_node.branch_value = Some(value);
		branch_node
	}

	fn rlp_bytes(&mut self, db: &mut HashMap<H256, Vec<u8>>) -> Vec<u8> {
		let mut list: Vec<RLPEncodeableWrapper> = Vec::new();
		let mut bytes = Vec::new();

		for child in self.children.iter_mut() {
			list.push(RLPEncodeableWrapper::Raw(child.rlp_bytes(db)));
		}
		match &self.branch_value {
			Some(value) => list.push(RLPEncodeableWrapper::Raw(value.rlp_bytes(db))),
			None => list.push(RLPEncodeableWrapper::String(Vec::default())),
		}
		reth_rlp::encode_list(&list, &mut bytes);

		let hash = mpt_hash(&bytes, db);
		println!("{hash:?}: {:x?}", bytes);
		hash
	}
}

impl From<BranchNode> for Node {
	fn from(value: BranchNode) -> Self {
		Node::Branch(value)
	}
}

impl Debug for BranchNode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// TODO: Use f.alternate()
		for (i, v) in self.children.iter().enumerate() {
			f.write_fmt(format_args!("{i:x}: {:#?}\n", v))?;
		}
		f.write_fmt(format_args!("value: {:#?}", self.branch_value))?;
		Ok(())
	}
}

enum RLPEncodeableWrapper {
	String(Vec<u8>),
	Raw(Vec<u8>),
}

impl Encodable for RLPEncodeableWrapper {
	fn encode(&self, out: &mut dyn reth_rlp::BufMut) {
		match self {
			Self::String(value) => Bytes::from(value.clone()).encode(out),
			Self::Raw(value) => out.put_slice(value),
		}
	}
}

// mpt_hash implements H(x) as used in the MPT.
fn mpt_hash(x: &[u8], db: &mut HashMap<H256, Vec<u8>>) -> Vec<u8> {
	// let x = format!("0x{}", hex::encode(x));
	// let x = x.as_bytes();
	if x.len() < 32 {
		x.to_vec()
	} else {
		let h = keccak256(&x);
		db.insert(h, x.to_vec());
		h.as_bytes().into()
	}
}

// match_paths is a helper function that returns the shared bytes between a & b as well
// the remaining bytes in a & b.
fn match_paths<'a, 'b>(key: &'a [u8], path: &'b [u8]) -> (Vec<u8>, &'a [u8], &'b [u8]) {
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
fn bytes_to_nibbles(key: &[u8]) -> Vec<u8> {
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
fn nibbles_to_compact(nibbles: &[u8], extension: bool) -> Vec<u8> {
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
		first = first | key[0];
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
