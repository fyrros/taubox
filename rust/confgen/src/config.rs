use file_manager::FileManager;

use yaml_rust::{Yaml, YamlLoader};
use std::collections::HashMap;
use std::path::{PathBuf};


static CONFIG_DIR_PATH: &'static str = "conf";
static CORES_FILE_PATH: &'static str = "cores.yaml";
static SERVERS_FILE_PATH: &'static str = "servers.yaml";
static LOGIC_COMMON_FILE_PATH: &'static str = "logic/common.yaml";
static LOGIC_CORES_DIR_PATH: &'static str = "logic/cores";
static TEMPLATES_XML_DIR_PATH: &'static str = "templates/xml";


#[derive(Debug)]
pub struct Config {
    pub cores: Yaml,
    servers: Yaml,
    logic_common: Yaml,
    logic_cores: HashMap<String, Yaml>,
    templates_xml: HashMap<String, String>,
}

impl Config {

    pub fn new() -> Config {

        let config_loader = ConfigLoader::new();

        Config {
            cores: config_loader.load_yaml_file(CORES_FILE_PATH),
            servers: config_loader.load_yaml_file(SERVERS_FILE_PATH),
            logic_common: config_loader.load_yaml_file(LOGIC_COMMON_FILE_PATH),
            logic_cores: config_loader.load_yaml_dir(LOGIC_CORES_DIR_PATH),
            templates_xml: config_loader.load_xml_dir(TEMPLATES_XML_DIR_PATH),
        }

    }
}


#[derive(Debug)]
struct ConfigLoader {
    config_dir: PathBuf
}

impl ConfigLoader {
    pub fn new() -> ConfigLoader {
        ConfigLoader {
            config_dir: PathBuf::from(CONFIG_DIR_PATH)
        }
    }

    pub fn load_yaml_file(&self, path_str: &str) -> Yaml {
        let path = self.get_path(path_str);
        self.load(path)
    }

    pub fn load_yaml_dir(&self, path_str: &str) -> HashMap<String, Yaml> {
        let path = self.get_path(path_str);
        self.load_dir(path)
    }

    pub fn load_xml_dir(&self, path_str: &str) -> HashMap<String, String> {
        let path = self.get_path(path_str);
        self.load_dir(path)
    }

    fn get_path(&self, path_str: &str) -> PathBuf {
        let mut path = PathBuf::from(&self.config_dir);
        path.push(path_str);
        path
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



