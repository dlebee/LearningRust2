use std::{collections::HashMap};

use ethers::{contract::stream, providers::Middleware};
use tokio::time;
use tokio_stream::{StreamExt, StreamMap};

use crate::{network::{Network}, network_service::{NetworkService}};

pub struct BlockWatcherService {
    network_service: NetworkService,
    latest_blocks: HashMap<ethers::types::U256, u64>
}

pub async fn listen_for_blocks(networks: HashMap<ethers::types::U256, Network>) -> Result<(), String> {

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
                println!("stream received a new block {} for chain id {}", block.number.unwrap(), chain_id);
            },
            _ = time::sleep(time::Duration::from_millis(1000), ) =>{

            }
        }
    }

    Ok(())
}

impl BlockWatcherService {
    pub async fn try_initialize(network_service: NetworkService) -> Result<Self, String> {
        
        let latest_block_map: HashMap<ethers::types::U256, u64> = HashMap::new();

        let networks = network_service.get_networks();
        let _ = tokio::spawn(async move {
            listen_for_blocks(networks).await;
        });

        Ok(Self {
            network_service: network_service,
            latest_blocks: latest_block_map
        })
    }   
}