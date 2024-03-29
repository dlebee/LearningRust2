use std::{sync::{atomic::AtomicBool}, time::Duration};
use std::sync::{Arc};
use chrono::{DateTime, Local};

use ethers::{abi::AbiEncode, providers::{Http, Middleware, Provider, Ws}, types::{H160}};
use std::sync::Mutex;
use tokio::{sync::mpsc::{self, Sender}, time};
use tokio_stream::{wrappers::ReceiverStream, StreamExt, StreamMap};

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
    pub latest_block: u64,
    pub last_saved_time: DateTime<Local>
}

pub struct NetworkService {
    pub networks: Arc<Mutex<Vec<Network>>>,
    stopping: Arc<AtomicBool>
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
            latest_block: 0,
            last_saved_time: Local::now()
        })
    }
}

pub async fn listen_for_blocks(pairs: Vec<(NetworkConfiguration, u64, Provider<Ws>)>, sender: Sender<(u64, u64)>, stop: Arc<AtomicBool>) -> Result<(), String> {
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

                match sender.send((cloned_chain_id, cloned_block_number)).await {
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

        let should_stop = stop.load(std::sync::atomic::Ordering::Relaxed);
        if should_stop {
            break;
        }
    }

    return Ok(());
}

pub async fn get_transfers(
    arc_networks: Arc<Mutex<Vec<Network>>>,
    stop: Arc<AtomicBool>,
    receiver: mpsc::Receiver<(u64, u64)>,
) -> Result<(), String> {
    // Convert the receiver into a stream outside the loop
    let mut stream = ReceiverStream::new(receiver);

    loop {
        tokio::select! {
            // Process messages from the stream
            Some(pair) = stream.next() => {
                let (chain_id, block_number) = pair;

                let mut matched_http_provider: Option<Provider<Http>> = None;
                let mut chain_name: String = String::from("Unknown");

                {
                    let mut networks = arc_networks.lock().unwrap();
                    for network in &mut *networks {
                        if network.chain_id == chain_id {

                            let previous_network_block = network.latest_block;
                            network.latest_block = block_number;

                            if previous_network_block > 0 {

                                // we have a previous block lets calculate the time since.
                                let now = Local::now();
                                let duration = now.signed_duration_since(network.last_saved_time);
                                network.last_saved_time = now;

                                println!("📦 New block picked up, chainId {}, name: {}, block: {}, previous block received: {}, time since: {}",
                                            chain_id, network.config.name.clone(), block_number, previous_network_block, duration);
                            } else {


                                println!("📦 New block picked up, chainId {}, name: {}, block: {}, previous block received: N/A, time since: N/A",
                                            chain_id, network.config.name.clone(), block_number);
                            }

                            matched_http_provider = Some(network.http.clone());
                            chain_name = network.config.name.clone();
                            break;
                        }
                    }
                }

                match matched_http_provider {
                    Some(http_provider) => {

           
                        let block_option = http_provider.get_block_with_txs(block_number).await.unwrap();
                        match block_option {
                            Some(block) => {
                                for transaction in block.transactions {
                                    if transaction.input.len() == 0 {
                                        println!("💵 New Transfer for chain: {} hash: {}, index: {:?}, from: {}, to: {}, ctc transfered: {}", 
                                            chain_name,
                                            transaction.hash.encode_hex(),
                                            transaction.transaction_index,
                                            transaction.from.encode_hex(),
                                            transaction.to.unwrap_or_else(|| H160::zero()).encode_hex(),
                                            transaction.value
                                        );
                                    }
                                }
                            }, 
                            None => {
                                eprintln!("could not find block {} on chain {}", block_number, chain_id);
                            }
                        }

                    },
                    None => {

                    }
                }
            },
            // Check the stop condition
            _ = tokio::task::yield_now() => {
                if stop.load(std::sync::atomic::Ordering::Relaxed) {
                    break;
                }
            }
        }
    }

    Ok(())
}

impl NetworkService {

    pub async fn try_initialize(network_configurations: Vec<NetworkConfiguration>) -> Result<Self, String> {

        let mut networks: Vec<Network> = Vec::new();

        let (sender, receiver) = mpsc::channel::<(u64, u64)>(10);

        for network_configuration in network_configurations {
            let network = Network::try_initialize(network_configuration).await?;
            networks.push(network);
        }

        let mut chain_and_provider: Vec<(NetworkConfiguration, u64, Provider<Ws>)> = Vec::new();
        for network in &mut networks {
            chain_and_provider.push((network.config.clone(), network.chain_id, network.wss.clone()));
        }

        let arc_networks = Arc::new(Mutex::new(networks));

        let arc_stop = Arc::new(AtomicBool::new(false));
        let arc_cloned = arc_stop.clone();
        let _ = tokio::spawn(async move { 
            listen_for_blocks(chain_and_provider, sender, arc_cloned).await 
        });

        let cloned_arc_networks_for_get_transfers = arc_networks.clone();
        let cloned_stop_for_get_transfers = arc_stop.clone();
        let _ = tokio::spawn(async move {
            let _ = get_transfers(cloned_arc_networks_for_get_transfers, cloned_stop_for_get_transfers, receiver).await;
        });

        Ok(Self {
            networks: arc_networks,
            stopping: arc_stop
        })
    }

    pub async fn cleanup(&mut self) {
        self.stopping.store(true, std::sync::atomic::Ordering::Release);
    }
}
