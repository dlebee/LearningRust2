use std::collections::HashMap;

use crate::network::{Network, NetworkConfiguration};

#[derive(Debug)]
pub struct NetworkService {
    networks: HashMap<ethers::types::U256, Network>
}

impl NetworkService {
    pub async fn try_initialize(network_configurations: Vec<NetworkConfiguration>) -> Result<Self, String> {

        let mut network_map: HashMap<ethers::types::U256, Network> = HashMap::new();

        for network_configuration in network_configurations {
            let network = Network::try_initialize(network_configuration).await?;
            if network_map.contains_key(&network.chain_id) {
                return Err(format!("There is already a network with the chain id {} initialized", network.chain_id));
            }

            network_map.insert(network.chain_id, network);
        }

        Ok(Self {
            networks: network_map
        })
    }

    pub fn get_network_by_name(self, name: String) -> Option<Network> {
        for (_chain_id, network) in self.networks {
            if network.config.name == name {
                return Some(network);
            }
        }

        None
    }

    pub fn get_network_by_chainid(self, chain_id: ethers::types::U256) -> Option<Network> {
        let network = self.networks.get(&chain_id);
        match network {
            Some(network) => {
                return Some(network.clone());
            },
            None => {
                return None;
            }
        }
    }

    pub fn get_networks(&self) -> HashMap<ethers::types::U256, Network> {
        self.networks.clone()
    }


}

