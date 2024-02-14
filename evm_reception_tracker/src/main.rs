mod network;

use crate::network::{NetworkConfiguration, NetworkService};
use tokio::signal;

#[tokio::main]
async fn main() {

    let mut network_service = NetworkService::try_initialize(vec![
        NetworkConfiguration {
            name: String::from("Creditcoin Testnet"),
            rpc: String::from("https://rpc.cc3-testnet.creditcoin.network"),
            wss: String::from("wss://rpc.cc3-testnet.creditcoin.network"),
        },
        NetworkConfiguration {
            name: String::from("Creditcoin Devnet"),
            rpc: String::from("https://rpc.cc3-devnet.creditcoin.network"),
            wss: String::from("wss://rpc.cc3-devnet.creditcoin.network")
        },
    ]).await.unwrap();

    // Create a stream for Ctrl+C signals
    println!("Running. Press Ctrl+C to exit.");
    let mut _ctrl_c = signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");

    network_service.cleanup().await;
}