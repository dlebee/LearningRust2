use std::collections::HashMap;
use crate::configuration_service::{ConfigurationService};
use dotenv::dotenv;
use nject::provider;
use crate::network_service::NetworkService;

mod configuration_service;
mod network_service;

#[provider]
struct Provider;

fn main() {
    dotenv().ok();
    let mut network_service: NetworkService = Provider.provide();
    println!("{:?}", network_service.networks);
    network_service.initialize();
    println!("{:?}", network_service.networks);
}
