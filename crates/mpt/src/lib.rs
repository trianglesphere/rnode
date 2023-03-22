use crate::misc::*;
use core::types::H256;
use reth_primitives::keccak256;
use std::{collections::HashMap, fmt::Debug};

mod display;
mod misc;
#[cfg(test)]
mod test;

#[derive(Default)]
pub struct MPT {
	root: Node,
	db: HashMap<H256, Vec<u8>>,
}

impl MPT {
	pub fn hash(&mut self) -> H256 {
		keccak256(self.root.rlp_bytes(&mut self.db))
	}

	pub fn insert(&mut self, k: Vec<u8>, v: Vec<u8>) {
		let k = bytes_to_nibbles(&k);
		let root = std::mem::take(&mut self.root);
		self.root = root.insert(&k, v);
	}

	pub fn get(&self, k: Vec<u8>) -> Option<&[u8]> {
		self.root.get(&bytes_to_nibbles(&k))
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
	fn new(nibbles: &[u8], child: Node) -> Self {
		if nibbles.is_empty() {
			child
		} else {
			ExtensionNode::new_node(nibbles.to_owned(), Box::new(child))
		}
	}

	fn new_value(value: Vec<u8>) -> Self {
		Node::Value(ValueNode::new(value))
	}

	fn insert(self, nibbles: &[u8], value: Vec<u8>) -> Self {
		match self {
			Node::Empty => Node::new(nibbles, Node::new_value(value)),
			Node::Branch(node) => node.insert(nibbles, value),
			Node::Extension(node) => node.insert(nibbles, value),
			Node::Value(..) => Node::new_value(value),
		}
	}

	fn get(&self, nibbles: &[u8]) -> Option<&[u8]> {
		match self {
			Node::Empty => None,
			Node::Branch(node) => node.get(nibbles),
			Node::Extension(node) => node.get(nibbles),
			Node::Value(node) => node.get(nibbles),
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

#[derive(Default)]
struct BranchNode {
	children: [Box<Node>; 16],
	branch_value: Option<ValueNode>,
}

impl BranchNode {
	// inserts adds a key/value to a branch node as either a sub-node or as a value.
	// It returns none if there is an error.
	fn insert(mut self, nibbles: &[u8], value: Vec<u8>) -> Node {
		if nibbles.is_empty() {
			self.branch_value = Some(ValueNode::new(value));
		} else {
			let i = nibbles[0] as usize;
			*self.children[i] = std::mem::take(&mut self.children[i]).insert(&nibbles[1..], value);
		};
		self.into()
	}

	fn get(&self, nibbles: &[u8]) -> Option<&[u8]> {
		if nibbles.is_empty() {
			self.branch_value.as_ref().map(|v| &v.value[..])
		} else {
			self.children[nibbles[0] as usize].get(&nibbles[1..])
		}
	}

	// new_with_node creates a new branch node that contains a given child node.
	fn new_with_node(key: u8, node: Box<Node>) -> Self {
		let mut branch_node = BranchNode::default();
		branch_node.children[key as usize] = node;
		branch_node
	}

	// new_with_value creates a new branch node that contains the given value.
	fn new_with_value(value: ValueNode) -> Self {
		BranchNode {
			branch_value: Some(value),
			..Default::default()
		}
	}

	fn rlp_bytes(&mut self, db: &mut HashMap<H256, Vec<u8>>) -> Vec<u8> {
		let mut list: Vec<RLPEncodeableWrapper> = Vec::new();
		let mut bytes = Vec::new();
		for child in self.children.iter_mut() {
			list.push(mpt_hash(&child.rlp_bytes(db), db));
		}
		match &self.branch_value {
			Some(value) => list.push(mpt_hash(&value.rlp_bytes(db), db)),
			None => list.push(RLPEncodeableWrapper::EmptyString),
		}
		reth_rlp::encode_list(&list, &mut bytes);
		bytes
	}
}

impl From<BranchNode> for Node {
	fn from(value: BranchNode) -> Self {
		Node::Branch(value)
	}
}

struct ExtensionNode {
	nibbles: Vec<u8>,
	child: Box<Node>,
}

impl ExtensionNode {
	fn new_node(nibbles: Vec<u8>, child: Box<Node>) -> Node {
		Node::Extension(Self { nibbles, child })
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

	fn insert(self, nibbles: &[u8], value: Vec<u8>) -> Node {
		let (common, new_nibbles, old_nibbles) = match_paths(nibbles, &self.nibbles);
		if new_nibbles.is_empty() && old_nibbles.is_empty() {
			return self.child.insert(nibbles, value);
		}
		// Inserting here will alwasy create branch node.
		// Turn the existing node into that branch node then insert the new value.
		let branch_node = if old_nibbles.is_empty() {
			match *self.child {
				Node::Empty => panic!("Cannot point to an empty node in an extension"),
				Node::Extension(..) => panic!("Cannot point to an extension node in an extension node"),
				Node::Value(child) => BranchNode::new_with_value(child),
				Node::Branch(child) => child,
			}
		} else {
			let child = Box::new(Node::new(&old_nibbles[1..], *(self.child)));
			BranchNode::new_with_node(old_nibbles[0], child)
		}
		.insert(new_nibbles, value);
		// Create an extension node based on the common part if needed.
		if common.is_empty() {
			branch_node
		} else {
			ExtensionNode::new_node(common, Box::new(branch_node))
		}
	}

	fn get(&self, nibbles: &[u8]) -> Option<&[u8]> {
		let (_, new_nibbles, old_nibbles) = match_paths(nibbles, &self.nibbles);
		if old_nibbles.is_empty() {
			self.child.get(new_nibbles)
		} else {
			None
		}
	}

	fn rlp_bytes(&mut self, db: &mut HashMap<H256, Vec<u8>>) -> Vec<u8> {
		let mut bytes = Vec::new();
		let list = vec![RLPEncodeableWrapper::Bytes(self.compact()), mpt_hash(&self.child.rlp_bytes(db), db)];
		reth_rlp::encode_list(&list, &mut bytes);
		bytes
	}
}

struct ValueNode {
	value: Vec<u8>,
}

impl ValueNode {
	fn new(value: Vec<u8>) -> Self {
		Self { value }
	}
	fn get(&self, _nibbles: &[u8]) -> Option<&[u8]> {
		// TODO: Intentional bug to see if fuzzing will catch it
		Some(&self.value)
	}
	fn rlp_bytes(&self, _: &mut HashMap<H256, Vec<u8>>) -> Vec<u8> {
		encode_bytes(self.value.clone())
	}
}
