use crate::{BranchNode, ExtensionNode, ValueNode, MPT};
use std::fmt::Debug;

impl Debug for MPT {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("root: {:#?}\n", &self.root))?;
		for (k, v) in self.db.iter() {
			f.write_fmt(format_args!("{k:?}\t0x{}\n", hex::encode(v)))?;
		}
		Ok(())
	}
}

impl Debug for BranchNode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// TODO: Use f.alternate()
		for (i, v) in self.children.iter().enumerate() {
			f.write_fmt(format_args!("{i:x}: {v:#?}\n"))?;
		}
		f.write_fmt(format_args!("value: {:#?}", self.branch_value))?;
		Ok(())
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

impl Debug for ValueNode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{:x?}", &self.value))?;
		Ok(())
	}
}
