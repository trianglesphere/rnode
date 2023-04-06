#![no_main]

use libfuzzer_sys::fuzz_target;
use mpt::MPT;
use std::collections::HashMap;

fuzz_target!(|input: Vec<(Vec<u8>, Vec<u8>)>| {
	let mut map = HashMap::new();
	let mut mpt = MPT::default();
	for (k, v) in input.iter() {
		map.insert(k.clone(), v.clone());
		mpt.insert(k.clone(), v.clone());
	}
	for (k, v) in map {
		let stored = mpt.get(k.clone()).unwrap();
		assert_eq!(stored, &v[..], "MPT value != input value");
	}
});
