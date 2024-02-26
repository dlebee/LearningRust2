use crate::configuration_service::{Configuration, ConfigurationService};
use dotenv::dotenv;
use crate::network_service::NetworkService;

mod configuration_service;
mod network_service;

fn main() {
    dotenv().ok();
    let configuration_service = ConfigurationService::new();
    let network_service = NetworkService::new(configuration_service);
    println!("{:?}", network_service);
}
