use crate::*;
use std::str::FromStr;

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

#[test]
fn test_empty_root_hash() {
	let mut mpt = MPT::new();
	let hash = mpt.hash();
	let expected = H256::from_str("56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421").unwrap();
	assert_eq!(hash, expected);
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
// Script to generate the hashes
// // go 1.19
// // require github.com/ethereum/go-ethereum v1.11.2
// package main

// import (
// 	"fmt"

// 	"github.com/ethereum/go-ethereum/trie"
// 	"github.com/ethereum/go-ethereum/core/rawdb"
// )

// func main() {
// 	// ('do', 'verb'), ('dog', 'puppy'), ('doge', 'coin'), ('horse', 'stallion').
// 	t := trie.NewEmpty(trie.NewDatabase(rawdb.NewMemoryDatabase()))
// 	t.Update([]byte("do"), []byte("verb"))
// 	t.Update([]byte("dog"), []byte("puppy"))
// 	t.Update([]byte("doge"), []byte("coin"))
// 	t.Update([]byte("horse"), []byte("stallion"))
// 	fmt.Println(t.Hash())
// }
#[test]
fn test_mpt_hash() {
	let mut mpt = MPT::new();

	mpt.insert("do".into(), "verb".into());
	let hash = mpt.hash();
	let expected_hash = H256::from_str("0x014f07ed95e2e028804d915e0dbd4ed451e394e1acfd29e463c11a060b2ddef7").unwrap();
	assert_eq!(expected_hash, hash);

	mpt.insert("dog".into(), "puppy".into());
	let hash = mpt.hash();
	let expected_hash = H256::from_str("0x779db3986dd4f38416bfde49750ef7b13c6ecb3e2221620bcad9267e94604d36").unwrap();
	assert_eq!(expected_hash, hash);

	mpt.insert("doge".into(), "coin".into());
	let hash = mpt.hash();
	let expected_hash = H256::from_str("0xef7b2fe20f5d2c30c46ad4d83c39811bcbf1721aef2e805c0e107947320888b6").unwrap();
	assert_eq!(expected_hash, hash);

	mpt.insert("horse".into(), "stallion".into());
	let hash = mpt.hash();
	let expected_hash = H256::from_str("0x5991bb8c6514148a29db676a14ac506cd2cd5775ace63c30a4fe457715e9ac84").unwrap();
	assert_eq!(expected_hash, hash);
}
