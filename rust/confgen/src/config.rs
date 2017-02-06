use file_manager::*;

use types::*;
use yaml_rust::{Yaml, YamlLoader};
use std::collections::{HashMap, BTreeMap};
use std::path::PathBuf;


static CORES_FILE_PATH: &'static str        = "conf/cores.yaml";
static SERVERS_FILE_PATH: &'static str      = "conf/servers.yaml";
static TEMPLATES_FILE_PATH: &'static str    = "conf/templates.yaml";
static LOGIC_COMMON_FILE_PATH: &'static str = "conf/logic/common.yaml";
static LOGIC_CORES_DIR_PATH: &'static str   = "conf/logic/cores";


#[derive(Debug)]
pub struct Config {
    cores: Yaml,
    servers: Yaml,
    logic_common: Yaml,
    logic_cores: HashMap<Id, Yaml>,
    templates: Yaml,
}

impl Config {

    pub fn new() -> Config {

        let config_loader = ConfigLoader::new();

        Config {
            cores: config_loader.load_file(CORES_FILE_PATH),
            servers: config_loader.load_file(SERVERS_FILE_PATH),
            logic_common: config_loader.load_file(LOGIC_COMMON_FILE_PATH),
            logic_cores: config_loader.load_dir(LOGIC_CORES_DIR_PATH),
            templates: config_loader.load_file(TEMPLATES_FILE_PATH),
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

    pub fn get_core_logic(&self, core_id: &Id) -> &Yaml {
        if let Some(yaml_config) = self.logic_cores.get(core_id) {
            yaml_config
        } else {
            panic!("Logic for {:?} not found!", core_id);
        }
    }

    pub fn get_template(&self, template_name: &str) -> &str {
        if let Some(template) = self.templates[template_name].as_str() {
            template
        } else {
            panic!("Template with name '{:?}' not found", template_name);
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

impl FilePath for ConfigLoader{}

impl FileLoader<String, String> for ConfigLoader {

    fn convert(&self, file_str: String) -> String {
        file_str
    }

    fn get_file_key(&self, entry_path: &PathBuf) -> String {
        entry_path.file_stem().unwrap().to_str().unwrap().to_string()
    }
}

impl FileLoader<Id, Yaml> for ConfigLoader {

    fn convert(&self, file_str: String) -> Yaml {
        let docs = YamlLoader::load_from_str(&file_str).unwrap();
        docs[0].clone()
    }

    fn get_file_key(&self, entry_path: &PathBuf) -> Id {
        entry_path.file_stem().unwrap().to_str().unwrap().parse().unwrap()
    }
}

#[derive(Debug)]
pub struct ConfigFile {
    path: String,
    body: String,
}

impl ConfigFile {
    
    pub fn new(path_string: String, body: String) -> ConfigFile {
        ConfigFile {
            path: path_string,
            body: body,
        }
    }

    pub fn save_config(&self) {
        self.save_file(self.path.as_str(), self.body.as_str());
    }
}

impl FilePath for ConfigFile{}
impl FileSaver for ConfigFile{}