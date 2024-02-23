use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use ethers::abi::{AbiEncode};
use ethers::middleware::Middleware;
use futures::FutureExt;
use tokio::time;
use tokio_stream::{StreamExt, StreamMap};
use crate::background_service::BackgroundService;
use crate::network::Network;
use crate::network_service::NetworkService;

pub struct PendingTransactionWatcherService {
    pub stopping: Arc<AtomicBool>,
}

async fn listen_for_pending_transactions(networks: HashMap<ethers::types::U256, Network>, stopping: Arc<AtomicBool>) -> Result<(), String> {

    // subscribe
    let mut stream_map = StreamMap::new();
    for (chain_id, network) in &networks {
        let pending_transactions_stream = network.wss_provider.watch_pending_transactions().await;
        match pending_transactions_stream {
            Ok(stream) => {
                stream_map.insert(chain_id, stream);
            },
            Err(e) => {
                return Err(format!("failed to create stream for watching pending transactions for network {} and chainid {}, error: {}", network.config.name, chain_id, e));
            }
        }
    }

    loop {
        if stopping.load(Ordering::Relaxed) {
            break;
        }

        tokio::select! {
            Some((chain_id, pending_transaction_hash)) = stream_map.next() => {
                // this gives you a transaction hash.
                match networks.get(chain_id) {
                    Some(network) => {
                        let transaction_future = network.http_provider
                            .get_transaction(pending_transaction_hash).await;

                        match transaction_future {
                            Ok(pending_transaction_opt) => {
                                match pending_transaction_opt {
                                    Some(transaction) => {
                                        println!("💵 New Pending transaction for chain: {} hash: {}, index: {:?}, from: {}, to: {}, ctc transfered: {}",
                                            network.config.name,
                                            transaction.hash.encode_hex(),
                                            transaction.transaction_index,
                                            transaction.from.encode_hex(),
                                            transaction.to.unwrap_or_default().encode_hex(),
                                            transaction.value
                                        );
                                    },
                                    None => {
                                        println!("⚠️ could not find pending transaction on chain {} hash {}", chain_id, pending_transaction_hash);
                                    }
                                }
                            },
                            Err(e) => {
                                eprintln!("failed to fetch transaction from provider of chain {} with hash {}, error: {}", chain_id, pending_transaction_hash, e);
                            }
                        }

                    },
                    None => {
                        eprintln!("received a pending transaction for a unknown network {} transaction hash {}", chain_id, pending_transaction_hash);
                    }
                }
            },
            _ = time::sleep(time::Duration::from_millis(1000), ) =>{

            }
        }
    }

    Ok(())
}

impl BackgroundService for PendingTransactionWatcherService {
    fn cleanup(&self) -> Pin<Box<dyn Future<Output = ()>>> {
        self.stopping.store(true, std::sync::atomic::Ordering::Release);
        async {
        }.boxed()
    }
}

impl PendingTransactionWatcherService {

    pub async fn try_initialize(network_service: NetworkService) -> Result<Self, String> {

        let stopping = Arc::new(AtomicBool::new(false));

        let stopping_clone = stopping.clone();
        let networks = network_service.get_networks();
        let _ = tokio::spawn(async move {
            let _ = listen_for_pending_transactions(networks, stopping_clone).await;
        });

        Ok(Self {
            stopping
        })
    }
}