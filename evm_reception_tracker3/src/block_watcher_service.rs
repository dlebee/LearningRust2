use std::{collections::HashMap, sync::{Arc}};
use ethers::providers::Middleware;
use tokio::{sync::{broadcast::{self, Sender}, Mutex}};
use tokio_stream::{wrappers::BroadcastStream, StreamMap, StreamExt};
use tokio_util::sync::CancellationToken;

use crate::{network::Network, network_service::NetworkService};

#[derive(Debug)]
pub struct BlockWatcherService {
    pub latest_blocks: Arc<Mutex<HashMap<ethers::types::U256, u64>>>,
    pub block_rx: broadcast::Receiver<(ethers::types::U256, u64)>
}

pub async fn listen_for_blocks(networks: HashMap<ethers::types::U256, Network>, 
    block_rx: Sender<(ethers::types::U256, u64)>,
    stop_token: CancellationToken) -> Result<(), String> {

    let mut stream_map = StreamMap::new();
    for (chain_id, network) in &networks {

        let block_stream = network.wss_provider.subscribe_blocks().await;
        match block_stream {
            Ok(stream) => {
                stream_map.insert(chain_id.clone(), stream);
            },
            Err(e) => {
                return Err(format!("failed to create block subscription for network {}, error: {}", network.config.name, e));
            }
        }
    }

    loop {

        tokio::select! {
            Some((chain_id, block)) = stream_map.next() => {
                match block_rx.send((chain_id, block.number.unwrap().as_u64())) {
                    Ok(_) => {

                    },
                    Err(e) => {
                        eprintln!("failed to publish block {} for chain id {}, error: {}",
                            block.number.unwrap(), chain_id, e);
                    }
                }
            },
            _ = stop_token.cancelled() => {
                break;
            }
        }
    }

    Ok(())
}

impl BlockWatcherService {

    pub async fn try_initialize(network_service: NetworkService, stop_token: CancellationToken) -> Result<Self, String> {

        let latest_block_map: HashMap<ethers::types::U256, u64> = HashMap::new();
        let latest_block_map_arc_mutex = Arc::new(Mutex::new(latest_block_map));

        let (block_tx, block_rx) = broadcast::channel::<(ethers::types::U256, u64)>(100);

        let networks = network_service.get_networks();
        let stop_token_clone = stop_token.clone();
        let _ = tokio::spawn(async move {
            let _ = listen_for_blocks(networks, block_tx, stop_token_clone).await;
        });

        let cloned_block_rx = block_rx.resubscribe();
        let cloned_arc = latest_block_map_arc_mutex.clone();
        let clone_stop_token_latest_block = stop_token.clone();

        tokio::spawn(async move {
            
            let mut stream = BroadcastStream::new(cloned_block_rx);
            loop {

                tokio::select! {
                    Some(block_stream_result) = stream.next() => {
                        match block_stream_result {
                            Ok((chain_id, block_number)) => {
                                let mut locked_block_map = cloned_arc.lock().await;
                                locked_block_map.insert(chain_id, block_number);
                                
                            },
                            Err(err) => {
                                eprintln!("Failed to receive block number from stream to update latest block map, err: {}", err);
                            }
                        }
                    },
                    _ = clone_stop_token_latest_block.cancelled() => {
                        break;
                    }
                }
            }
        });

        Ok(Self {
            latest_blocks: latest_block_map_arc_mutex,
            block_rx
        })
    }   
}