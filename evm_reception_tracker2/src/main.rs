mod network;
mod network_service;
mod block_watcher_service;

use block_watcher_service::BlockWatcherService;
use network_service::NetworkService;
use tokio::signal;

use crate::network::{NetworkConfiguration};

#[tokio::main]
async fn main() -> Result<(), String> {

    let network_service = NetworkService::try_initialize(vec![
        NetworkConfiguration {
            name: String::from("Creditcoin Devnet"),
            http: String::from("https://rpc.cc3-devnet.creditcoin.network"),
            wss: String::from("wss://rpc.cc3-devnet.creditcoin.network")
        },
        NetworkConfiguration {
            name: String::from("Creditcoin Testnet"),
            http: String::from("https://rpc.cc3-testnet.creditcoin.network"),
            wss: String::from("wss://rpc.cc3-testnet.creditcoin.network")
        },
    ]).await?;

    let block_watcher = BlockWatcherService::try_initialize(network_service).await?;

        // Create a stream for Ctrl+C signals
    println!("Running. Press Ctrl+C to exit.");
    let mut _ctrl_c = signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");

    Ok(())
}
