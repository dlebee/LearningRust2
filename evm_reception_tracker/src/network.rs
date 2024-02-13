use ethers::providers::{Provider, Ws, Http, Middleware};
use tokio_stream::{StreamExt, StreamMap, Stream};
use ethers::types::U256;

pub struct NetworkConfiguration {
    pub name: String,
    pub wss: String,
    pub rpc: String
}


#[derive(Debug)]
pub struct Network {
    pub name: String,
    pub wss: String,
    pub rpc: String,

    pub initialized: Option<InitializedNetwork>
}

#[derive(Debug)]
pub struct InitializedNetwork {
    pub wss: Provider<Ws>,
    pub http: Provider<Http>,
    pub chain_id: u64
}

pub struct NetworkService {
    pub networks: Vec<Network>


}

impl Network {
    pub async fn initialize(&mut self) {
        println!("⚡ Initializing network {} creating web socket provider...", self.name);
        let wss_provider = Provider::<Ws>::connect(self.wss.to_string()).await;
        println!("⚡ Initializing network {} creating http provider...", self.name);
        let http_provider = Provider::<Http>::try_from(self.rpc.to_string());

        let wss_provider_unwrapped = wss_provider.unwrap();
        let http_provider_unwrapped = http_provider.unwrap();


        println!("🔢 Getting chain id for network {}", self.name);
        let wss_chain_id = wss_provider_unwrapped.get_chainid().await.unwrap();
        let http_chain_id = http_provider_unwrapped.get_chainid().await.unwrap();

        if wss_chain_id != http_chain_id {
            panic!("Should be the same chain id between http and wss {} != {}", wss_chain_id, http_chain_id);
        }

        println!("🔢 Chain ID for {} is {}", self.name, http_chain_id);

        self.initialized = Some(InitializedNetwork{
            wss: wss_provider_unwrapped,
            http: http_provider_unwrapped,
            chain_id: wss_chain_id.as_u64()
        });
    }
}

pub async fn listen_for_blocks(pairs: Vec<(u64, String, Provider<Ws>)>) {
    let mut map = StreamMap::new();
    for pair in &pairs {
        let stream = pair.2.subscribe_blocks().await.unwrap();
        map.insert(pair.1.clone(), stream);
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
    pub fn new(network_configurations: Vec<NetworkConfiguration>) -> Self {

        let networks: Vec<Network> = network_configurations.into_iter().map(|network| {
            Network {
                name: network.name,
                wss: network.wss,
                rpc: network.rpc,
                initialized: None
            }
        }).collect();

        Self {
            networks
        }
    }


    pub async fn initialize(&mut self) {

        let mut chain_and_provider: Vec<(u64, String, Provider<Ws>)> = Vec::new();

        for network in &mut self.networks {
            network.initialize().await;

            match &network.initialized {
                Some(initialized) => {
                    let pair = (initialized.chain_id, network.name.clone(), initialized.wss.clone());
                    chain_and_provider.push(pair);
                },
                None => {
                    eprintln!("network {} was not initialized", network.name);
                }
            };
        }

        if chain_and_provider.len() > 0 {
            let _network_block_watcher = tokio::spawn(async move { listen_for_blocks(chain_and_provider).await });
        }
    }

    pub async fn cleanup(&mut self) {


    }
}