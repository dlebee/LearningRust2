use std::fmt::format;
use std::sync::Arc;
use ethers::providers::{Provider, Ws, Http, Middleware};
use tokio_stream::{StreamExt, StreamMap, Stream};
use ethers::types::U256;

#[derive(Debug)]
#[derive(Clone)]
pub struct NetworkConfiguration {
    pub name: String,
    pub wss: String,
    pub rpc: String
}

#[derive(Debug)]
pub struct Network {

    pub config: NetworkConfiguration,
    pub wss: Provider<Ws>,
    pub http: Provider<Http>,
    pub chain_id: u64
}

pub struct NetworkService {
    pub networks: Vec<Network>
}

impl Network {
    pub async fn try_initialize(network_configuration: NetworkConfiguration) -> Result<Self, String> {

        println!("⚡ Initializing network {} creating web socket provider...", network_configuration.name);
        let wss_provider;
        match Provider::<Ws>::connect(network_configuration.wss.to_string()).await {
            Ok(wss_provider_result) => {
                wss_provider = wss_provider_result;
            },
            Err(e) => {
                return Err(format!("failed to initialize wss provider {}", e));
            }
        }

        println!("⚡ Initializing network {} creating http provider...", network_configuration.name);
        let http_provider;
        match Provider::<Http>::try_from(network_configuration.rpc.to_string()) {
            Ok(http_provider_result) => {
                http_provider = http_provider_result;
            },
            Err(e) => {
                return Err(format!("failed to initialize http provider {}", e));
            }
        }


        println!("🔢 Getting chain id for network {} using websocket", network_configuration.name);
        let wss_chain_id;
        match wss_provider.get_chainid().await {
            Ok(chain_id) => {
                wss_chain_id = chain_id
            },
            Err(e) => {
                return Err(format!("failed to get websocket chain id for network {}, error: {}", network_configuration.name, e));
            }
        }

        println!("🔢 Getting chain id for network {} using https", network_configuration.name);
        let http_chain_id;
        match http_provider.get_chainid().await {
            Ok(chain_id) => {
                http_chain_id = chain_id
            },
            Err(e) => {
                return Err(format!("failed to get http chain id for network {}, error: {}", network_configuration.name, e));
            }
        }

        if wss_chain_id != http_chain_id {
            return Err(format!("Should be the same chain id between http and wss {} != {}", wss_chain_id, http_chain_id));
        }

        println!("🔢 Chain ID for {} is {}", network_configuration.name, http_chain_id);
        Ok(Self {
            config: network_configuration,
            chain_id: http_chain_id.as_u64(),
            wss: wss_provider,
            http: http_provider
        })
    }
}

pub async fn listen_for_blocks(pairs: Vec<(NetworkConfiguration, Provider<Ws>)>) -> Result<(), String> {
    let mut map = StreamMap::new();
    for pair in &pairs {
        let (network_configuration, provider) = pair;

        match provider.subscribe_blocks().await {
            Ok(stream) => {
                map.insert(network_configuration.name.clone(), stream);
            },
            Err(e) => {
                return Err(format!("failed to create block subscription for network {}", network_configuration.name));
            }
        }
    }

    loop {
        tokio::select! {
            Some((name, block)) = map.next() => {
                println!("📦 New block for {} is {}", name, block.number.unwrap());
            }
        }
    }
}

impl NetworkService {

    pub async fn try_initialize(network_configurations: Vec<NetworkConfiguration>) -> Result<Self, String> {

        let mut networks: Vec<Network> = Vec::new();

        for network_configuration in network_configurations {
            let network = Network::try_initialize(network_configuration).await?;
            networks.push(network);
        }

        let mut chain_and_provider: Vec<(NetworkConfiguration, Provider<Ws>)> = Vec::new();
        for network in &mut networks {
            chain_and_provider.push((network.config.clone(), network.wss.clone()));
        }

        if chain_and_provider.len() > 0 {
            let _network_block_watcher = tokio::spawn(async move { listen_for_blocks(chain_and_provider).await });
        }

        Ok(Self {
            networks
        })
    }

    pub async fn cleanup(&mut self) {


    }
}