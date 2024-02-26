use std::collections::HashMap;
use std::env;

const SEPARATOR: &str = "__";

#[derive(Clone, Debug)]
pub enum Configuration {
    Value(String),
    SubConfiguration(HashMap<String, Configuration>),
}

impl Configuration {
    fn as_sub_configuration_mut(&mut self) -> &mut HashMap<String, Configuration> {
        if let Configuration::SubConfiguration(map) = self {
            map
        } else {
            panic!("Expected SubConfiguration");
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConfigurationService {
    settings: HashMap<String, Configuration>,
}

impl ConfigurationService {
    pub fn new() -> Self {
        let mut settings = HashMap::new();

        for (key, value) in env::vars() {
            let key_parts: Vec<&str> = key.split(SEPARATOR).collect();
            let mut current_parent = &mut settings;

            for (index, key_part) in key_parts.iter().enumerate() {
                let is_last_index = index == key_parts.len() - 1;
                let string_key_part = String::from(*key_part);

                if is_last_index {
                    current_parent.insert(string_key_part, Configuration::Value(value.clone()));
                } else {
                    current_parent = current_parent
                        .entry(string_key_part.clone())
                        .or_insert_with(|| Configuration::SubConfiguration(HashMap::new()))
                        .as_sub_configuration_mut();
                }
            }
        }

        Self { settings }
    }
}
