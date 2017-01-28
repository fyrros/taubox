use file_manager::FileManager;

use yaml_rust::{Yaml, YamlLoader};
use std::collections::HashMap;


static CORES_FILE_PATH: &'static str        = "conf/cores.yaml";
static SERVERS_FILE_PATH: &'static str      = "conf/servers.yaml";
static LOGIC_COMMON_FILE_PATH: &'static str = "conf/logic/common.yaml";
static LOGIC_CORES_DIR_PATH: &'static str   = "conf/logic/cores";
static TEMPLATES_XML_DIR_PATH: &'static str = "conf/templates/xml";


#[derive(Debug)]
pub struct Config {
    cores: Yaml,
    servers: Yaml,
    logic_common: Yaml,
    logic_cores: HashMap<String, Yaml>,
    templates_xml: HashMap<String, String>,
}

impl Config {

    pub fn new() -> Config {

        let config_loader = ConfigLoader::new();

        Config {
            cores: config_loader.load_file(CORES_FILE_PATH),
            servers: config_loader.load_file(SERVERS_FILE_PATH),
            logic_common: config_loader.load_file(LOGIC_COMMON_FILE_PATH),
            logic_cores: config_loader.load_dir(LOGIC_CORES_DIR_PATH),
            templates_xml: config_loader.load_dir(TEMPLATES_XML_DIR_PATH),
        }

    }
}


#[derive(Debug)]
struct ConfigLoader;

impl ConfigLoader {
    pub fn new() -> ConfigLoader {
        ConfigLoader
    }
}

impl FileManager<String> for ConfigLoader {
    fn convert(&self, file_str: String) -> String {
        file_str
    }
}

impl FileManager<Yaml> for ConfigLoader {
    fn convert(&self, file_str: String) -> Yaml {
        let docs = YamlLoader::load_from_str(&file_str).unwrap();
        docs[0].clone()
    }
}


