use ethers_core::abi::AbiDecode;
use ethers_core::types::H256;
use rs_node::client::*;

#[test]
pub fn test_get_block_with_receipts() {
	let rpc_url = "https://eth-goerli.g.alchemy.com/v2/ktzxXAN_NDkz_5ikfCnPf_TzoQXEF53m";
	let mut client = Client::new(rpc_url).unwrap();
	let hash = H256::decode_hex("0xee9dd94ebc06b50d5d5c0f72299a3cc56737e459ce41ddb44f0411870f86b1a3").unwrap();
	let block = client.get_block_with_receipts(hash).unwrap();

	assert_eq!(block.block.hash.unwrap(), hash);
}

#[test]
pub fn test_get_transaction_receipt() {
	// let rpc_url = "https://eth-goerli.g.alchemy.com/v2/ktzxXAN_NDkz_5ikfCnPf_TzoQXEF53m";
	// let client = Client::new(rpc_url).unwrap();
	// let hash = H256::decode_hex("0xee9dd94ebc06b50d5d5c0f72299a3cc56737e459ce41ddb44f0411870f86b1a3").unwrap();
	// let _receipt = client.get_transaction_receipt(hash).unwrap();

	// assert_eq!(receipt.transaction_hash, hash);
}
