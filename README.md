# dot_rebase

This project is intended to be a POC re-implementation of the Ampleforth, 'rebasing' crypto currency on the Substrate / Polkadot blockchain.

The main Ethereum / Solidity-based implmentation is here: https://github.com/ampleforth/uFragments

### Build + testing notes

* Use `./build.sh` helper script to build deployable code.

Note the output of the build step:

```
Your contract artifacts are ready. You can find them in:
/Users/pgo/code/oleary-labs/dot_rebase/target/ink

  - dot_rebase.contract (code + metadata)
  - dot_rebase.wasm (the contract's code)
  - metadata.json (the contract's metadata)
```

* These artifacts are what are deployed to the blockchain
  * The WASM is the compiled code that is deployed and runs on the blockchain.
  * The JSON / ABI is the metadata that tells the rest of the system how to interact with the WASM.

* Use the `./test.sh` helper script to run a local test blockchain.  
  * The script just runs this command: `canvas --dev --tmp`. Note that the `--tmp` flag means that the blockchain is only temporary.
  If you stop and restart it it loses all state. I presume that running without it *will* maintain state but haven't tested this way yet.
* With the blockchain running locally, the 'default' Polkadot UI seems to be test best way to operate the blockchain: https://polkadot.js.org/apps/#/explorer
* The control in the upper left corner of the UI allows you to pick the chain to operate. Choose 'Local Node' under 'DEVELOPMENT' to use the local test instance.
* Under the 'Develop' tab at the top, choosing 'Contract' will get the UI that allows you to deploy and operate the compiled contract code.
  * Remember that the account that you use to operate the blockchain needs to be funded.
  * Also, the UI itself seems to store some state about the contracts. If you run the local node with `--tmp` the previously-deployed code will not be there even though it shows in the UI.
  The 'status' then shows as 'Not on-chain'. You'll need to remove the contract from the UI and re-deploy.