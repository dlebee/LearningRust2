mod network;
mod network_service;
mod block_watcher_service;
mod transfer_watcher_service;
mod pending_transaction_watcher_service;

use futures::future::join_all;
use block_watcher_service::BlockWatcherService;
use network_service::NetworkService;
use tokio::signal;

use crate::{network::NetworkConfiguration, transfer_watcher_service::TransferWatcherService};
use crate::pending_transaction_watcher_service::PendingTransactionWatcherService;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> Result<(), String> {

    let stop_token = CancellationToken::new();

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

    let block_watcher = BlockWatcherService::try_initialize(network_service.clone(), stop_token.clone()).await?;
    let transfer_watcher = TransferWatcherService::try_initialize(network_service.clone(), block_watcher.block_rx.resubscribe(), stop_token.clone()).await?;
    let pending_transaction_watcher = PendingTransactionWatcherService::try_initialize(network_service.clone(), stop_token.clone()).await?;

        // Create a stream for Ctrl+C signals
    println!("Running. Press Ctrl+C to exit.");
    let _ctrl_c = signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");

    stop_token.cancel();

    let handles = vec![
        block_watcher.latest_block_handle,
        block_watcher.block_listener_handle,
        transfer_watcher.handle,
        pending_transaction_watcher.handle
    ];

    join_all(handles).await;

    println!("latest block when the application stopped {:?}", block_watcher.latest_blocks);
    Ok(())
}
