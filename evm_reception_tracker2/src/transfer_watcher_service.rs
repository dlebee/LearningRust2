use std::{cell::Cell, sync::Arc};

use ethers::{abi::AbiEncode, providers::Middleware};
use tokio_stream::{wrappers::{BroadcastStream, ReceiverStream}, StreamExt};

use crate::{block_watcher_service::BlockWatcherService, network_service::NetworkService};

pub struct TransferWatcherService {}

impl TransferWatcherService {
    pub async fn try_initialize(
        network_service: NetworkService,
        block_watcher_service: BlockWatcherService,
    ) -> Result<Self, String> {
        let mut stream = BroadcastStream::new(block_watcher_service.block_rx);

        loop {
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

        Ok(Self {})
    }
}
