def deploy(self, keypair, endowment, gas_limit, constructor, 
           args: dict = None, deployment_salt: str = None, 
           upload_code: bool = False) -> ContractInstance 
    

    data = self.metadata.generate_constructor_data(name=constructor, args=args) 

    if upload_code is true: 

        if.self.substrate.get_metadata_call_function('Contracts', 'Instantiate_with_code') is not None: 

            if not self.wasm_bytes: 
                raise ValueError("No WASM bytes to upload") 

            call = self.substrate.compose_call( 
                call_module='Contracts',
                call_function='instantiate_with_code', 
                call_params={ 
                        'endowment': endowment, 
                        'gas limit': gas_limit, 
                        'code': '0x{}'.format(self.wasm_bytes.hex()), 
                        'data': data.to_hex(), 
                        'salt': deployment_salt or " 
                } 

            ) 
        else: 
            

             self.upload_wasm(keypair)

             call= self.substrate.compose_call( 
                 call_module='Contracts', 
                 call_function='instantiate', 
                 call_params={ 
                     'endowment': endowment, 
                     'gas_limit': gas_limit, 
                     'code_hash': f'0xx{self.code_hash.hex()}',
                     'data': data.to_hex(), 
                     'salt': deployment_salt or " 
                 } 

             ) 
            
    else: 
    
             call = self.substrate.compose_call( 
                 call_module='Contracts', 
                 call_function='instantiate', 
                 call_params={ 
                     'endowment': endowment, 
                     'gas_limit': gas_limit, 
                     'code_hash': f'0x{self.code_hash.hex()}', 
                     'data': data.to_hex(), 
                     'salt': deployment_salt or " 
                 } 
                ) 
    
    extrinsic = self.substrate.create_signed_extrinsic(call=call, keypair=keypair) 
    
    result = self.substrate.submit_extrinsic(extrinsic, wait_for_inclusion=True)
    
    if not result.is_success: 
        raise ExtrinsicFaialedException(result.error_message) 

    for event in result.triggered_events: 
        if event.event.name == 'Instantiated': 
                return ContractInstance( 
                    contract_address=event.params[1]['value'], 
                    metadata=self.metadata, 
                    substrate=self.substrate 
                ) 
    
    raise DeployContractFailedException()
                