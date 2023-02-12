use revm::{
	inspectors::CustomPrintTracer,
	primitives::{Address, BlockEnv, Bytes, CfgEnv, EVMError, Env, ExecutionResult, SpecId, TxEnv, U256},
	InMemoryDB, EVM,
};

/// The execution module contains logic for executing transactions.
pub struct Executor {
	/// The internal evm
	evm: EVM<InMemoryDB>,
	/// The internal inspector
	inspector: CustomPrintTracer,
}

impl Default for Executor {
	fn default() -> Self {
		Self::new()
	}
}

impl Executor {
	/// Creates a new instance of the executor.
	pub fn new() -> Self {
		let database = InMemoryDB::default();
		let mut evm = EVM::new();
		evm.database(database);
		Self {
			evm,
			inspector: CustomPrintTracer::default(),
		}
	}

	/// Builds a new evm execution environment.
	pub fn build_env(&mut self, caller: Address, to: Address, data: Vec<u8>) -> eyre::Result<()> {
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
		self.evm.env = env;

		Ok(())
	}

	/// Inspects and commits the current state of the evm.
	pub fn inspect_commit(&mut self) -> std::result::Result<ExecutionResult, EVMError<std::convert::Infallible>> {
		// Inspect the current state of the evm
		self.evm.inspect_commit(&mut self.inspector)
	}

	// Execute a block.
	// pub fn execute_block(block: &Block) -> eyre::Result<()> {
	// 	Err(eyre::eyre!("Not implemented"))
	// }
}
