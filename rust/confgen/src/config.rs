use file_manager::FileManager;

use types::*;
use yaml_rust::{Yaml, YamlLoader};
use std::collections::{HashMap, BTreeMap};
use std::path::PathBuf;


static CORES_FILE_PATH: &'static str             = "conf/cores.yaml";
static SERVERS_FILE_PATH: &'static str           = "conf/servers.yaml";
static LOGIC_COMMON_FILE_PATH: &'static str      = "conf/logic/common.yaml";
static LOGIC_CORES_DIR_PATH: &'static str        = "conf/logic/cores";
static TEMPLATES_SERVERS_FILE_PATH: &'static str = "conf/templates/servers.yaml";
static TEMPLATES_COPIES_FILE_PATH: &'static str  = "conf/templates/copies.yaml";


#[derive(Debug)]
pub struct Config {
    cores: Yaml,
    servers: Yaml,
    logic_common: Yaml,
    logic_cores: HashMap<Id, Yaml>,
    templates: Templates,
}

impl Config {

    pub fn new() -> Config {

        let config_loader = ConfigLoader::new();

        Config {
            cores: config_loader.load_file(CORES_FILE_PATH),
            servers: config_loader.load_file(SERVERS_FILE_PATH),
            logic_common: config_loader.load_file(LOGIC_COMMON_FILE_PATH),
            logic_cores: config_loader.load_dir(LOGIC_CORES_DIR_PATH),
            templates: config_loader.load_templates(),
        }
    }

    pub fn get_cores(&self) -> &BTreeMap<Yaml, Yaml> {
        self.cores.as_hash().unwrap()
    }

    pub fn get_servers(&self) -> &BTreeMap<Yaml, Yaml> {
        self.servers.as_hash().unwrap()
    }

    pub fn get_logic_common(&self) -> &Yaml {
        &self.logic_common
    }

    pub fn get_logic(&self, core_id: &Id) -> &Yaml {
        if let Some(yaml_config) = self.logic_cores.get(core_id) {
            yaml_config
        } else {
            println!("Logic for {:?} not found!", core_id);
            panic!();
        }
    }
}


#[derive(Debug)]
struct ConfigLoader;

impl ConfigLoader {
    pub fn new() -> ConfigLoader {
        ConfigLoader
    }

    pub fn load_templates(&self) -> Templates {
        Templates {
            copies: self.load_file(TEMPLATES_COPIES_FILE_PATH),
            servers: self.load_file(TEMPLATES_SERVERS_FILE_PATH),            
        }
    }
}

impl FileManager<String, String> for ConfigLoader {
    fn convert(&self, file_str: String) -> String {
        file_str
    }

    fn get_file_key(&self, entry_path: &PathBuf) -> String {
        entry_path.file_stem().unwrap().to_str().unwrap().to_string()
    }
}

impl FileManager<Id, Yaml> for ConfigLoader {
    fn convert(&self, file_str: String) -> Yaml {
        let docs = YamlLoader::load_from_str(&file_str).unwrap();
        docs[0].clone()
    }

    fn get_file_key(&self, entry_path: &PathBuf) -> Id {
        entry_path.file_stem().unwrap().to_str().unwrap().parse().unwrap()
    }
}


#[derive(Debug)]
struct Templates {
    copies: Yaml,
    servers: Yaml,
    //scripts: Yaml
}