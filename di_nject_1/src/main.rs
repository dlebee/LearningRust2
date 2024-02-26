use crate::configuration_service::ConfigurationService;
use dotenv::dotenv;

mod configuration_service;

fn main() {
    dotenv().ok();
    let _configuration_service = ConfigurationService::new();

    println!("{:?}", _configuration_service);
}
