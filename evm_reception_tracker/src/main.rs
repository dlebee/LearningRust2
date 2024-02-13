mod network;

use crate::network::{NetworkConfiguration, NetworkService};

#[tokio::main]
async fn main() {

    let mut network_service = NetworkService::new(vec![
        NetworkConfiguration {
            name: String::from("Creditcoin Testnet"),
            rpc: String::from("https://rpc.cc3-testnet.creditcoin.network"),
            wss: String::from("wss://rpc.cc3-testnet.creditcoin.network"),
        },
        NetworkConfiguration {
            name: String::from("Creditcoin Devnet"),
            rpc: String::from("https://rpc.cc3-devnet.creditcoin.network"),
            wss: String::from("wss://rpc.cc3-devnet.creditcoin.network")
        },
    ]);

    network_service.initialize().await;

    for network in network_service.networks {
        println!("name: {}, chain id: {}", network.name, network.initialized.unwrap().chain_id);
    }
}
