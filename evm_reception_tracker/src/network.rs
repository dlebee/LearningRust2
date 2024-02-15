use std::{sync::mpsc::{self, Sender}, time::Duration};
use std::sync::{Arc, Mutex};

use ethers::{providers::{Http, Middleware, Provider, Ws}};
use tokio::time;
use tokio_stream::{StreamExt, StreamMap};

#[derive(Debug)]
#[derive(Clone)]
pub struct NetworkConfiguration {
    pub name: String,
    pub wss: String,
    pub rpc: String
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Network {

    pub config: NetworkConfiguration,
    pub wss: Provider<Ws>,
    pub http: Provider<Http>,
    pub chain_id: u64,
    pub latest_block: u64
}

pub struct NetworkService {
    pub networks: Arc<Mutex<Vec<Network>>>,
    
    stopping: Arc<Mutex<bool>>
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
            http: http_provider,
            latest_block: 0
        })
    }
}

pub async fn listen_for_blocks(pairs: Vec<(NetworkConfiguration, u64, Provider<Ws>)>, sender: Sender<(u64, u64)>, stop: Arc<Mutex<bool>>) -> Result<(), String> {
    let mut map = StreamMap::new();
    for pair in &pairs {
        let (network_configuration, chain_id, provider) = pair;

        match provider.subscribe_blocks().await {
            Ok(stream) => {
                map.insert(chain_id, stream);
            },
            Err(e) => {
                return Err(format!("failed to create block subscription for network {}, error: {}", network_configuration.name, e));
            }
        }
    }

    loop {
        tokio::select! {
            Some((chain_id, block)) = map.next() => {
                let cloned_chain_id = chain_id.clone();
                let cloned_block_number = block.number.unwrap().as_u64();


                match sender.send((cloned_chain_id, cloned_block_number)) {
                    Ok(()) => {

                    },
                    Err(e) => {
                        eprint!("failed to send through channel the new block, chain id: {} new block: {}, error: {}", 
                            cloned_chain_id, cloned_block_number, e);
                    }
                }
            },
            _ = time::sleep(Duration::from_millis(1000), ) =>{

            }
        }

        let should_stop = *stop.lock().unwrap();
        if should_stop {
            break;
        }
    }

    return Ok(());
}

impl NetworkService {

    pub async fn try_initialize(network_configurations: Vec<NetworkConfiguration>) -> Result<Self, String> {

        let mut networks: Vec<Network> = Vec::new();

        let (sender, receiver) = mpsc::channel::<(u64, u64)>();

        for network_configuration in network_configurations {
            let network = Network::try_initialize(network_configuration).await?;
            networks.push(network);
        }

        let mut chain_and_provider: Vec<(NetworkConfiguration, u64, Provider<Ws>)> = Vec::new();
        for network in &mut networks {
            chain_and_provider.push((network.config.clone(), network.chain_id, network.wss.clone()));
        }

        let arc_networks = Arc::new(Mutex::new(networks));

        let arc_stop = Arc::new(Mutex::new(false));

        let arc_cloned = arc_stop.clone();
        let _network_block_watcher = tokio::spawn(async move { 
            listen_for_blocks(chain_and_provider, sender, arc_cloned).await 
        });

        let cloned_arc = arc_networks.clone();
        let arc_stop_cloned2 = arc_stop.clone();
        let _block_updater = tokio::spawn(async move {
            loop {
                match receiver.recv_timeout(Duration::from_millis(1000)) {
                    Ok((chain_id, block_number)) => {

                        let mut networks = cloned_arc.lock().unwrap();
                        for network in &mut *networks {
                            if network.chain_id == chain_id {

                                let previous_network_block = network.latest_block;
                                network.latest_block = block_number;

                                if previous_network_block > 0 {
                                    println!("📦 New block picked up, chainId {}, name: {}, block: {}, previous block received: {}",
                                                chain_id, network.config.name.clone(), block_number, previous_network_block);
                                } else {
                                    println!("📦 New block picked up, chainId {}, name: {}, block: {}, previous block received: N/A",
                                                chain_id, network.config.name.clone(), block_number);
                                }
                            }
                        }

                    },
                    Err(_) => {

                    }
                }

                let should_stop = *arc_stop_cloned2.lock().unwrap();
                if should_stop {
                    break;
                }
            } 
        });

        Ok(Self {
            networks: arc_networks,
            stopping: arc_stop
        })
    }

    pub async fn cleanup(&mut self) {
        *self.stopping.lock().unwrap() = true;
    }
}