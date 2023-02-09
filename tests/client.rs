use std::str::FromStr;

use ethers_core::abi::AbiDecode;
use ethers_core::types::H256;
use rs_node::client::*;

#[test]
pub fn test_get_header() {
	let rpc_url = "https://eth-goerli.g.alchemy.com/v2/ktzxXAN_NDkz_5ikfCnPf_TzoQXEF53m";
	let mut client = Client::new(rpc_url).unwrap();
	let hash = H256::decode_hex("0xee9dd94ebc06b50d5d5c0f72299a3cc56737e459ce41ddb44f0411870f86b1a3").unwrap();
	let header = client.get_header(hash).unwrap();
	let expected_hash = reth_primitives::H256::from(hash.as_fixed_bytes());
	assert_eq!(header.hash_slow(), expected_hash);
}

#[test]
pub fn test_get_transactions_by_root() {
	let rpc_url = "https://eth-goerli.g.alchemy.com/v2/ktzxXAN_NDkz_5ikfCnPf_TzoQXEF53m";
	let mut client = Client::new(rpc_url).unwrap();

	// We shouldn't be able to get transactions for a block that doesn't exist
	let expected_transactions_root_hash = H256::from_str("0x9c2887743fb87670295f144bb2b82b47e7a2b446f116a5be967c597dd3a9c60c").unwrap();
	let missing_txs = client.get_transactions_by_root(expected_transactions_root_hash);
	assert!(missing_txs.is_err());

	// Now let's load the header, then fetch the transactions by the transactions root hash
	let hash = H256::decode_hex("0xee9dd94ebc06b50d5d5c0f72299a3cc56737e459ce41ddb44f0411870f86b1a3").unwrap();
	let header = client.get_header(hash).unwrap();
	let tx_root_hash = ethers_core::types::H256::from(header.transactions_root.as_fixed_bytes());
	let transactions = client.get_transactions_by_root(tx_root_hash).unwrap();
	assert_eq!(transactions.len(), 8);
}

#[test]
pub fn test_get_receipts_by_root() {
	let rpc_url = "https://eth-goerli.g.alchemy.com/v2/ktzxXAN_NDkz_5ikfCnPf_TzoQXEF53m";
	let mut client = Client::new(rpc_url).unwrap();

	// We shouldn't be able to get receipts for a block that doesn't exist
	let expected_receipts_root_hash = H256::from_str("0x4ee5bd14490683c247268211c96b8bf1ddc52aa8a209676cb7db147deebff9b0").unwrap();
	let missing_receipts = client.get_receipts_by_root(expected_receipts_root_hash);
	assert!(missing_receipts.is_err());

	// Now let's load the header, then fetch the receipts by the receipts root hash
	let hash = H256::decode_hex("0xee9dd94ebc06b50d5d5c0f72299a3cc56737e459ce41ddb44f0411870f86b1a3").unwrap();
	let header = client.get_header(hash).unwrap();
	let receipt_root_hash = ethers_core::types::H256::from(header.receipts_root.as_fixed_bytes());
	assert_eq!(receipt_root_hash, expected_receipts_root_hash);
	let receipts = client.get_receipts_by_root(receipt_root_hash).unwrap();
	assert_eq!(receipts.len(), 8);
}
