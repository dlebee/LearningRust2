use ethers::providers::{Provider, Ws, Http, Middleware};
use ethers::core::types::BlockId;

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
        println!("Initializing network {} creating web socket provider...", self.name);
        let wss_provider = Provider::<Ws>::connect(self.wss.to_string()).await;
        let http_provider = Provider::<Http>::try_from(self.rpc.to_string());

        let wss_provider_unwrapped = wss_provider.unwrap();
        let http_provider_unwrapped = http_provider.unwrap();

        let chain_id = http_provider_unwrapped.get_chainid().await;

        self.initialized = Some(InitializedNetwork{
            wss: wss_provider_unwrapped,
            http: http_provider_unwrapped,
            chain_id: chain_id.unwrap().as_u64()
        });
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

        for network in &mut self.networks {
            network.initialize().await;
        }
    }
}