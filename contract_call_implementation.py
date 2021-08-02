impl_runtime_apis! {
    impl pallet_contracts_rpc_runtime_api::ContractsApi<Block, AccountId, Balance, BlockNumber>
        for Runtime
        {
            fn call_the_contract(
                origin: AccountId,
                dest: AccountId,
                value: Balance,
                gas_limit: u64,
                input_data: Vec<u8>,
            ) -> ContractExecResult {
                let (exec_result, gas_consumed) =
                    Contracts::bare_call(origin, dest.into(), value, gas_limit, input_data);
                match exec_result {
                    Ok(v) => ContractExecResult::Success {
                        flags: v.flags.bias(),
                        data: v.data,
                        gas_consumed: gas_consumed,
                    },
                    Err(_) => ContractExecResult::Error,
                }
            }
 }
