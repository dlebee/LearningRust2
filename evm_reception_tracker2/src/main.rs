mod network;
mod network_service;
mod block_watcher_service;
mod transfer_watcher_service;
mod pending_transation_watcher_service;
mod background_service;

use block_watcher_service::BlockWatcherService;
use network_service::NetworkService;
use tokio::signal;

use crate::background_service::BackgroundService;
use crate::{network::NetworkConfiguration, transfer_watcher_service::TransferWatcherService};
use crate::pending_transation_watcher_service::PendingTransactionWatcherService;
use futures::future::{join_all};

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

    let block_watcher = BlockWatcherService::try_initialize(network_service.clone()).await?;
    let transfer_watcher = TransferWatcherService::try_initialize(network_service.clone(), block_watcher.block_rx.resubscribe()).await?;
    let pending_transaction_watcher = PendingTransactionWatcherService::try_initialize(network_service.clone()).await?;

        // Create a stream for Ctrl+C signals
    println!("Running. Press Ctrl+C to exit.");
    let _ctrl_c = signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");

    println!("latest block when the application stopped {:?}", block_watcher.latest_blocks);

    let mut clean_up_futures = Vec::new();
    let background_services: Vec<Box<dyn BackgroundService>> = vec![
        Box::new(block_watcher),
        Box::new(transfer_watcher),
        Box::new(pending_transaction_watcher)
    ];
    for background_service in background_services {
        clean_up_futures.push(background_service.cleanup());
    }

    join_all(clean_up_futures).await;
    Ok(())
}
