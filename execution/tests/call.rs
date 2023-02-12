use revm::primitives::{Address, EVMError, Eval, ExecutionResult, InvalidTransaction};

#[test]
fn test_call() {
	let caller = Address::zero();
	let to = Address::zero();
	let data = vec![0u8; 32];

	// Build the executor
	let mut executor = execution::Executor::new();
	executor.build_env(caller, to, data).unwrap();

	// Send a CALL transaction
	let call_res = executor.inspect_commit();
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
