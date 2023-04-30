This was project was built by adapting the package structure from the 
example repo for the ["Name Service" smart contract (link)](https://github.com/deus-labs/cw-contracts/tree/main/contracts/nameservice).

I copied that contract here as well, under the 'wasm_nameservice_sample_code' folder.  Please ignore it.

"Bonus" fee structure for contract owner is implemented under ExecuteMsg::TransferWithTip.
The contract owner can withdraw their fees using the same mechanism as for other users.


### To Run
In the base directory, please execute:
```
cargo test
```

