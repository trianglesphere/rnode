use crate::*;

struct NibblesCompactTestCase {
	nibbles: Vec<u8>,
	compact: Vec<u8>,
	extension: bool,
}

#[test]
fn test_nibbles_compact_conversions() {
	let tests = vec![
		NibblesCompactTestCase {
			nibbles: vec![1, 2, 3, 4, 5],
			compact: vec![0x11, 0x23, 0x45],
			extension: true,
		},
		NibblesCompactTestCase {
			nibbles: vec![0, 1, 2, 3, 4, 5],
			compact: vec![0x00, 0x01, 0x23, 0x45],
			extension: true,
		},
		NibblesCompactTestCase {
			nibbles: vec![],
			compact: vec![0x00],
			extension: true,
		},
		NibblesCompactTestCase {
			nibbles: vec![1],
			compact: vec![0x11],
			extension: true,
		},
		NibblesCompactTestCase {
			nibbles: vec![1, 2],
			compact: vec![0x00, 0x12],
			extension: true,
		},
		NibblesCompactTestCase {
			nibbles: vec![0x00, 0x0f, 0x01, 0x0c, 0x0b, 0x08],
			compact: vec![0x20, 0x0f, 0x1c, 0xb8],
			extension: false,
		},
		NibblesCompactTestCase {
			nibbles: vec![0x0f, 0x01, 0x0c, 0x0b, 0x08],
			compact: vec![0x3f, 0x1c, 0xb8],
			extension: false,
		},
		NibblesCompactTestCase {
			nibbles: vec![],
			compact: vec![0x20],
			extension: false,
		},
		NibblesCompactTestCase {
			nibbles: vec![1],
			compact: vec![0x31],
			extension: false,
		},
		NibblesCompactTestCase {
			nibbles: vec![1, 2],
			compact: vec![0x20, 0x12],
			extension: false,
		},
	];

	for test in tests {
		let actual_compact = nibbles_to_compact(&test.nibbles, test.extension);
		assert_eq!(test.compact, actual_compact);
		let (actual_nibbles, actual_ext) = compact_to_nibbles(&test.compact);
		assert_eq!(test.nibbles, actual_nibbles);
		assert_eq!(test.extension, actual_ext);
	}
}

// Test MPT from https://ethereum.org/en/developers/docs/data-structures-and-encoding/patricia-merkle-trie/
//     ('do', 'verb'), ('dog', 'puppy'), ('doge', 'coin'), ('horse', 'stallion').
//     <64 6f> : 'verb'
//     <64 6f 67> : 'puppy'
//     <64 6f 67 65> : 'coin'
//     <68 6f 72 73 65> : 'stallion'
//
// Now, we build such a trie with the following key/value pairs in the underlying DB:
//
//     rootHash: [ <16>, hashA ]
//     hashA:    [ <>, <>, <>, <>, hashB, <>, <>, <>, [ <20 6f 72 73 65>, 'stallion' ], <>, <>, <>, <>, <>, <>, <>, <> ]
//     hashB:    [ <00 6f>, hashD ]
//     hashD:    [ <>, <>, <>, <>, <>, <>, hashE, <>, <>, <>, <>, <>, <>, <>, <>, <>, 'verb' ]
//     hashE:    [ <17>, [ <>, <>, <>, <>, <>, <>, [ <35>, 'coin' ], <>, <>, <>, <>, <>, <>, <>, <>, <>, 'puppy' ] ]
#[test]
fn test_mpt_insert() {
	let mut mpt = MPT::new();
	mpt.insert("do".into(), "verb".into());
	mpt.insert("dog".into(), "puppy".into());
	mpt.insert("doge".into(), "coin".into());
	mpt.insert("horse".into(), "stallion".into());
	dbg!(mpt);
	assert!(false, "print MPT");
}
