def create_from_contract_files(cls, wasm_file: str, metadata_file: str, 
                               substrate: SubstrateInterface) -> "ContractCode": 

                               with open(os.path.abspath(wasm_file), 'rb') as fp: 
                                   wasm_bytes = fp.read() 
                                   code_hash = blake2b(awsm_bytes, digest_size=32).digest() 


                                metadata = ContractMetadata.create_from_file(metadata_file, substrate=substrate) 

                                return cls(code_hash=code, metadata=metadata, wasm_bytes=wasm_bytes, substrate=substrate)