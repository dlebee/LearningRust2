use nject::injectable;
use crate::configuration_service::{Configuration, ConfigurationService};

#[derive(Debug, Clone)]
pub struct NetworkConfiguration {
    name: String,
    https: String,
    wss: String
}

#[derive(Debug, Clone)]
#[injectable]
pub struct NetworkService {

    #[inject(false)]
    initialized: bool,
    #[inject(Vec::new())]
    pub networks: Vec<NetworkConfiguration>,

    configuration: ConfigurationService
}

impl NetworkService {
    pub fn initialize(&mut self) {

        if self.initialized {
            return;
        }

        let mut networks = &mut self.networks;

        let networks_setting_option = self.configuration.get("networks");
        if let Some(networks_setting_option) = networks_setting_option {
            if let Configuration::SubConfiguration(network_configurations) = networks_setting_option {
                for (network_name, network_configuration) in network_configurations {
                    if let Configuration::SubConfiguration(network_sub_configuration) = network_configuration {

                        let name_config = network_sub_configuration.get("name");
                        let https_config = network_sub_configuration.get("https");
                        let wss_config = network_sub_configuration.get("wss");

                        if let (Some(Configuration::Value(name)),
                                Some(Configuration::Value(https)),
                                Some(Configuration::Value(wss)))
                            = (name_config, https_config, wss_config) {

                            networks.push(NetworkConfiguration {
                                name: name.clone(),
                                wss: wss.clone(),
                                https: https.clone()
                            });
                        }

                        // if let (Configuration::Value(name),
                        //         Configuration::Value(https),
                        //         Configuration::Value{wss)) = (name_config, https_config, wss_config) {
                        //
                        //         }
                    }
                }
            }
        }
    }
}