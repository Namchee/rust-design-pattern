// Template Method is a design pattern that defines
// base algorithm in the superclass, but lets the subclass
// to have a finer grain control of it.
//
// In this example, we have a configuration manager that may
// read configuration file of different type JSON and YAML.
// In this case, the content parsing part is 100% same, but
// how the data is parsed is different and specific to each
// file type.

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub endpoint: String,
    pub timeout: u32,
}

pub trait ConfigurationManager {
    fn process_file(&self, path: Option<&str>) -> Configuration {
        let config_path = path.unwrap_or("path/to/config");

        println!("Reading configuration from {}", config_path);

        self.parse_file_content("<insert_file_data_here>")
    }

    fn parse_file_content(&self, content: &str) -> Configuration;
}

pub struct JSONConfigurationManager;
impl ConfigurationManager for JSONConfigurationManager {
    fn parse_file_content(&self, _: &str) -> Configuration {
        println!("Parsing JSON config...");

        serde_json::from_str("{ \"endpoint\": \"https://www.google.com\", \"timeout\": 5000 }").unwrap()
    }
}

pub struct YAMLConfigurationManager;
impl ConfigurationManager for YAMLConfigurationManager {
    fn parse_file_content(&self, _: &str) -> Configuration {
        println!("Parsing YAML config...");

        Configuration { endpoint: "https://twitter.com".to_string(), timeout: 10000 }
    }
}

pub fn parse_config(path: Option<&str>, manager: impl ConfigurationManager) -> Configuration {
    manager.process_file(path)
}
