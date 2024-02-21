use std::sync::{atomic::AtomicBool, Arc};

use ethers::{abi::AbiEncode, providers::Middleware};
use tokio::sync::broadcast::Receiver;
use tokio_stream::{wrappers::BroadcastStream, StreamExt};

use crate::network_service::NetworkService;

pub struct TransferWatcherService {
    stopping: Arc<AtomicBool>
}

async fn listen_for_transfers(network_service: NetworkService, block_rx: Receiver<(ethers::types::U256, u64)>, stopping: Arc<AtomicBool>) {
    
    let mut stream = BroadcastStream::new(block_rx);

    loop {

        if stopping.load(std::sync::atomic::Ordering::Relaxed) {
            break;
        }

        tokio::select! {
            // Process messages from the stream
            Some(result) = stream.next() => {

                match result {
                    Ok((chain_id, block_number)) => {
                        let network_opt = network_service.clone().get_network_by_chainid(chain_id);
                        match network_opt {
                            Some(network) => {

                                let block_option = network.http_provider.get_block_with_txs(block_number).await.unwrap();
                                match block_option {
                                    Some(block) => {
                                        if block.transactions.len() == 0 {
                                            println!("⚠️ could not find any transfers on block {} for chain {}", block_number, network.config.name);
                                        } else {
                                            for transaction in block.transactions {
                                                if transaction.input.len() == 0 {
                                                    println!("💵 New Transfer for chain: {} hash: {}, index: {:?}, from: {}, to: {}, ctc transfered: {}",
                                                        network.config.name,
                                                        transaction.hash.encode_hex(),
                                                        transaction.transaction_index,
                                                        transaction.from.encode_hex(),
                                                        transaction.to.unwrap_or_default().encode_hex(),
                                                        transaction.value
                                                    );
                                                }
                                            }
                                        }

                                    },
                                    None => {
                                        eprintln!("could not find block {} on chain {}", block_number, chain_id);
                                    }
                                }
                            },
                            None => {
                                eprintln!("could not find chain with id {} but received an event for block number {}", chain_id, block_number);
                            }
                        }
                    },
                    Err(err) => {
                        eprintln!("Broadcast error when trying to listen for block numbers, err: {}", err);
                    }
                }
            }
        }
    }
}

impl TransferWatcherService {

    pub async fn cleanup(self) {
        self.stopping.store(true, std::sync::atomic::Ordering::Release);
    }

    pub async fn try_initialize(
        network_service: NetworkService,
        block_rx: Receiver<(ethers::types::U256, u64)>
    ) -> Result<Self, String> {

        let stopping = Arc::new(AtomicBool::new(false));
        
        let stoping_clone = stopping.clone();
        tokio::spawn(async move {
            listen_for_transfers(network_service, block_rx, stoping_clone).await;
        });

        Ok(Self {
            stopping
        })
    }
}
