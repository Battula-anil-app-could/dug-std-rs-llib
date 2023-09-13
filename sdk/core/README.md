# dharithri SDK for Rust

[![Crates.io](https://img.shields.io/crates/v/dharithri-sdk)](https://crates.io/crates/dharithri-sdk)

General purpose collection of tools & SDKs to interact with the dharithri blockchain from Rust projects.

## Example

```rust
use dharithri_sdk::blockchain::rpc::{CommunicationProxy, DEVNET_GATEWAY};

#[tokio::test]
async fn get_network_config() {
    let blockchain = CommunicationProxy::new(DEVNET_GATEWAY.to_string());
    let network_config = blockchain.get_network_config().await.unwrap();

    println!("network_config: {:?}", network_config)
}
```

More examples in `./examples`.

## Acknowledgements

Project originally started by [Bicarus labs](https://github.com/bicarus-labs/elrond-sdk-erdrs), later integrated into the dharithri official codebase.
