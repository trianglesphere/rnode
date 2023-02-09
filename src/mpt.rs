use ethers_core::types::{Transaction, TransactionReceipt};
use reth_primitives::{
	proofs::{calculate_receipt_root, calculate_transaction_root},
	Log, Receipt, TransactionSigned, TxType, H256,
};

/// Get the unsecure receipt root from a list of [TransactionReceipt]s.
pub fn get_receipt_root(receipts: Vec<TransactionReceipt>) -> H256 {
	calculate_receipt_root(
		receipts.into_iter()
			.map(tx_receipt_to_reth_receipt)
			.collect::<Vec<Receipt>>()
			.iter(),
	)
}

/// Get the unsecure transaction root from a vec of [TransactionSigned]s.
pub fn get_transactions_root(transactions: Vec<Transaction>) -> H256 {
	calculate_transaction_root(
		transactions
			.into_iter()
			.map(tx_to_reth_tx)
			.collect::<Vec<TransactionSigned>>()
			.iter(),
	)
}

/// Convert a [TransactionReceipt] to a [Receipt].
fn tx_receipt_to_reth_receipt(receipt: TransactionReceipt) -> Receipt {
	let tx_type = match receipt.transaction_type {
		Some(tx_type) => match tx_type.as_u64() {
			1 => TxType::EIP2930,
			2 => TxType::EIP1559,
			_ => TxType::Legacy,
		},
		_ => TxType::Legacy,
	};

	let logs = receipt
		.logs
		.into_iter()
		.map(|log| Log {
			address: reth_primitives::H160::from(log.address.as_fixed_bytes()),
			topics: log
				.topics
				.into_iter()
				.map(|topic| reth_primitives::H256::from(topic.as_fixed_bytes()))
				.collect(),
			data: reth_primitives::Bytes(log.data.0),
		})
		.collect();

	Receipt {
		tx_type,
		bloom: reth_primitives::Bloom::from(receipt.logs_bloom.as_fixed_bytes()),
		cumulative_gas_used: receipt.cumulative_gas_used.as_u64(),
		logs,
		success: receipt.status.unwrap_or(reth_primitives::U64::from(0)).as_u64() == 1,
	}
}

/// Convert an [Transaction] to a [TransactionSigned].
fn tx_to_reth_tx(tx: Transaction) -> TransactionSigned {
	let chain_id = tx.chain_id.unwrap_or_default().as_u64();
	let nonce = tx.nonce.as_u64();
	let gas_price = tx.gas_price.unwrap_or_default().as_u128();
	let gas_limit = tx.gas.as_u64();
	let to = if let Some(to) = tx.to {
		reth_primitives::TransactionKind::Call(reth_primitives::H160::from(to.as_fixed_bytes()))
	} else {
		reth_primitives::TransactionKind::Create
	};
	let value = tx.value.as_u128();
	let input = reth_primitives::Bytes(tx.input.clone().0);
	let access_list: Option<reth_primitives::AccessList> = tx.access_list.as_ref().map(|access_list| {
		reth_primitives::AccessList(
			access_list
				.0
				.iter()
				.map(|item| reth_primitives::AccessListItem {
					address: reth_primitives::H160::from(item.address.as_fixed_bytes()),
					storage_keys: item
						.storage_keys
						.iter()
						.map(|key| reth_primitives::H256::from(key.as_fixed_bytes()))
						.collect(),
				})
				.collect::<Vec<reth_primitives::AccessListItem>>(),
		)
	});

	let inner_tx: reth_primitives::Transaction = match tx.transaction_type {
		Some(tx_type) => match tx_type.as_u64() {
			1 => reth_primitives::Transaction::Eip2930(reth_primitives::TxEip2930 {
				chain_id,
				nonce,
				gas_price,
				gas_limit,
				to,
				value,
				input,
				access_list: access_list.unwrap_or_default(),
			}),
			2 => reth_primitives::Transaction::Eip1559(reth_primitives::TxEip1559 {
				chain_id,
				nonce,
				gas_limit,
				max_fee_per_gas: tx.max_fee_per_gas.unwrap_or_default().as_u128(),
				max_priority_fee_per_gas: tx.max_priority_fee_per_gas.unwrap_or_default().as_u128(),
				to,
				value,
				access_list: access_list.unwrap_or_default(),
				input,
			}),
			_ => reth_primitives::Transaction::Legacy(reth_primitives::TxLegacy {
				chain_id: tx.chain_id.map(|cid| cid.as_u64()),
				nonce,
				gas_price,
				gas_limit,
				to,
				value,
				input,
			}),
		},
		None => panic!("Transaction type not specified"),
	};

	TransactionSigned {
		hash: reth_primitives::H256::from(tx.hash().as_fixed_bytes()),
		signature: reth_primitives::Signature {
			r: reth_primitives::U256::from_limbs(tx.r.0),
			s: reth_primitives::U256::from_limbs(tx.s.0),
			// An odd v means that the y-parity of the signature is true.
			odd_y_parity: tx.v.as_u64() % 2 == 1,
		},
		transaction: inner_tx,
	}
}
