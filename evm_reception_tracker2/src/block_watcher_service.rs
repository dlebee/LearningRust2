use std::{collections::HashMap, sync::{atomic::AtomicBool, Arc}};
use std::future::Future;
use std::pin::Pin;

use ethers::providers::Middleware;
use futures::FutureExt;
use tokio::{sync::{broadcast::{self, Sender}, Mutex}, time};
use tokio_stream::{wrappers::BroadcastStream, StreamExt, StreamMap};

use crate::{network::Network, network_service::NetworkService};
use crate::background_service::BackgroundService;

#[derive(Debug)]
pub struct BlockWatcherService {
    pub latest_blocks: Arc<Mutex<HashMap<ethers::types::U256, u64>>>,
    pub block_rx: broadcast::Receiver<(ethers::types::U256, u64)>,
    stopping: Arc<AtomicBool>
}

pub async fn listen_for_blocks(networks: HashMap<ethers::types::U256, Network>, 
    block_rx: Sender<(ethers::types::U256, u64)>,
    stopping: Arc<AtomicBool>) -> Result<(), String> {

    
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

        if stopping.load(std::sync::atomic::Ordering::Relaxed) {
            break;
        }

        tokio::select! {
            Some((chain_id, block)) = stream_map.next() => {
                match block_rx.send((chain_id, block.number.unwrap().as_u64())) {
                    Ok(_) => {

                    },
                    Err(e) => {
                        eprintln!("failed to publish block {} for chainid {}, error: {}", 
                            block.number.unwrap(), chain_id, e);
                    }
                }
            },
            _ = time::sleep(time::Duration::from_millis(1000), ) =>{

            }
        }
    }

    Ok(())
}

impl BlockWatcherService {

    pub async fn try_initialize(network_service: NetworkService) -> Result<Self, String> {
        
        let stopping = Arc::new(AtomicBool::new(false));

        let latest_block_map: HashMap<ethers::types::U256, u64> = HashMap::new();
        let latest_block_map_arc_mutex = Arc::new(Mutex::new(latest_block_map));

        let (block_tx, block_rx) = broadcast::channel::<(ethers::types::U256, u64)>(100);

        let networks = network_service.get_networks();
        let listen_for_blocks_stop = stopping.clone();
        let _ = tokio::spawn(async move {
            let _ = listen_for_blocks(networks, block_tx, listen_for_blocks_stop).await;
        });

        let cloned_block_rx = block_rx.resubscribe();
        let cloned_arc = latest_block_map_arc_mutex.clone();
        let clone_stop_update_latest_block = stopping.clone();

        tokio::spawn(async move {
            
            let mut stream = BroadcastStream::new(cloned_block_rx);
            loop {

                if clone_stop_update_latest_block.load(std::sync::atomic::Ordering::Relaxed) {
                    break;
                }

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
                    }
                }
                    
            }
        });

        Ok(Self {
            latest_blocks: latest_block_map_arc_mutex,
            block_rx,
            stopping
        })
    }   
}

impl BackgroundService for BlockWatcherService {
    fn cleanup(&self) -> Pin<Box<dyn Future<Output = ()>>> {
        self.stopping.store(true, std::sync::atomic::Ordering::Release);
        async {
        }.boxed()
    }
}