use revm::{
	inspectors::CustomPrintTracer,
	primitives::{Address, BlockEnv, Bytes, CfgEnv, EVMError, Env, Eval, ExecutionResult, InvalidTransaction, SpecId, TxEnv, U256},
	InMemoryDB, EVM,
};

#[test]
fn test_execution() {
	let caller = Address::zero();
	let to = Address::zero();
	let data = vec![0u8; 32];

	let mut evm: EVM<InMemoryDB> = EVM::new();
	let database = crate::InMemoryDB::default();
	let mut inspector = CustomPrintTracer::default();

	// Construct the environment
	let env = Env {
		cfg: CfgEnv {
			chain_id: U256::from(1),
			spec_id: SpecId::LATEST,
			..Default::default()
		},
		block: BlockEnv {
			basefee: U256::from(0),
			gas_limit: U256::MAX,
			..Default::default()
		},
		tx: TxEnv {
			caller,
			gas_limit: 0,
			gas_price: U256::from(0),
			gas_priority_fee: None,
			transact_to: revm::primitives::TransactTo::Call(to),
			value: U256::from(0),
			data: Bytes::from(data),
			chain_id: 1.into(),
			nonce: None,
			access_list: Vec::new(),

			// Added L2 fields
			is_fake: false,
			is_system_tx: false,
			is_deposit_tx: false,
			mint: U256::from(0),
			l1_cost_gas: 0,
		},
	};

	// Set the evm's environment
	evm.env = env;

	// Set the evm database
	evm.database(database);

	// Send a CALL transaction
	let call_res = evm.inspect_commit(&mut inspector);
	match call_res {
		Ok(result) => match result {
			ExecutionResult::Success {
				reason,
				gas_used,
				gas_refunded,
				logs,
				output: _,
			} => {
				assert_eq!(reason, Eval::Return);
				assert_eq!(gas_used, 0);
				assert_eq!(gas_refunded, 0);
				assert_eq!(logs, vec![]);
				// assert_eq!(output, Bytes::from(vec![0u8; 32]));
			}
			_ => panic!("Execution failed"),
		},
		Err(e) => {
			assert_eq!(e, EVMError::Transaction(InvalidTransaction::CallGasCostMoreThenGasLimit))
		}
	};
}
