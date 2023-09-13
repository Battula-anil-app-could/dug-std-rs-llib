# dharithri Smart Contract Framework

The crates in this folder form the dharithri smart contract framework.

They are as follows:
    - `dharithri-sc` - the base crate for smart contract libraries, it is the only dependency the smart contract code sees.
    - `dharithri-sc-derive` - procedural macros for friendlier SC code
    - `dharithri-sc-meta` - smart contract meta-programming: build system and other tools
    - `dharithri-sc-scenario` - the main testing tool, contracts are tested by via scenarios
    - `dharithri-sc-snippets` - base crate for tools that interact with the blockchain
    - `dharithri-sc-wasm-adapter` - the API that connects contracts to the WASM backend
