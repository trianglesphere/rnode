use backend::benchdb::BenchDb;
use backend::dblock::DBlock;
use backend::types::{BlockHash, BlockNumber, BlockWithReceipts, Database};
use ethers_core::abi::AbiDecode;
use ethers_core::types::Block;
use ethers_core::types::H256;

#[test]
pub fn test_phantom_read() {
	let block_hash: BlockHash = H256::decode_hex("1d2b0bda21d56b8bd12d4f94ebacffdfb35f5e226f84b461103bb8beab6353be").unwrap();
	let bencher = BenchDb::default();
	let mut database = DBlock::new(bencher);
	let block = database.read_block(block_hash).unwrap();
	assert_eq!(block.block.number, Some(BlockNumber::from(0)));
	assert_eq!(database.db.reads, 1);
}

#[test]
pub fn test_write() {
	let block_number = BlockNumber::from(42);
	let block_hash: BlockHash = H256::decode_hex("1d2b0bda21d56b8bd12d4f94ebacffdfb35f5e226f84b461103bb8beab6353be").unwrap();
	let bencher = BenchDb::default();
	let mut database = DBlock::new(bencher);
	database.write_block(BlockWithReceipts {
		block: Block {
			number: Some(block_number),
			hash: Some(block_hash),
			..Default::default()
		},
		receipts: vec![],
	})
	.unwrap();
}

#[test]
pub fn test_write_read() {
	let block_number = BlockNumber::from(42);
	let block_hash: BlockHash = H256::decode_hex("1d2b0bda21d56b8bd12d4f94ebacffdfb35f5e226f84b461103bb8beab6353be").unwrap();
	let bencher = BenchDb::default();
	let mut database = DBlock::new(bencher);
	database.write_block(BlockWithReceipts {
		block: Block {
			number: Some(block_number),
			hash: Some(block_hash),
			..Default::default()
		},
		receipts: vec![],
	})
	.unwrap();
	let block = database.read_block(block_hash).unwrap();
	assert_eq!(block.block.number, Some(block_number));
	assert_eq!(database.db.reads, 1);
}
