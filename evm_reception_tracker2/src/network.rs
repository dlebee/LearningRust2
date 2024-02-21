use std::sync::Arc;

use ethers::{providers::{Http, Middleware, Provider, Ws}};
use tokio::sync::Mutex;

#[derive(Debug)]
#[derive(Clone)]
pub struct NetworkConfiguration {
    pub name: String,
    pub wss: String,
    pub http: String
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Network {
    pub config: NetworkConfiguration,
    pub chain_id: ethers::types::U256,
    pub wss_provider: Arc<Provider<Ws>>,
    pub http_provider: Arc<Provider<Http>>
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
        match Provider::<Http>::try_from(network_configuration.http.to_string()) {
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
        
        Ok(Self {
            config: network_configuration,
            chain_id: http_chain_id,
            wss_provider: Arc::new(wss_provider),
            http_provider: Arc::new(http_provider)
        })
    }
}