use std::collections::HashMap;
use std::env;
use nject::injectable;

const SEPARATOR: &str = "__";
const QUERY_SEPARATOR: char = ':';

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
#[injectable]
pub struct ConfigurationService {
    #[inject(false)]
    initialized: bool,
    #[inject(HashMap::new())]
    settings: HashMap<String, Configuration>,
}

impl ConfigurationService {

    pub fn get(&mut self, key: &str) -> Option<&Configuration> {

        self.initialize();

        let mut result = None;
        let mut current_parent = &self.settings;

        let lower_case_key = key.to_lowercase();
        let parts: Vec<&str> = lower_case_key.split(QUERY_SEPARATOR).collect();
        let last_index = parts.len() - 1;

        for (index, key_part) in parts.iter().enumerate() {
            let string_key_part = String::from(*key_part);
            if last_index == index {
                return current_parent.get(&string_key_part);
            } else {
                match current_parent.get(&string_key_part) {
                    Some(result) => {
                        match result {
                            Configuration::SubConfiguration(map) => {
                                current_parent = map;
                            },
                            _ => {
                                // not suppose to happen...
                                return None;
                            }
                        }
                    },
                    None => {
                        return None;
                    }
                }
            }
        }

        result
    }

    pub fn initialize(&mut self) {

        if self.initialized {
            return;
        }

        let mut settings = HashMap::new();

        for (key, value) in env::vars() {
            let lower_case_key = key.to_lowercase();
            let key_parts: Vec<&str> = lower_case_key.split(SEPARATOR).collect();
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

        self.settings = settings;
    }
}
