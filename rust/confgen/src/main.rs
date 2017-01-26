extern crate yaml_rust;

use yaml_rust::{Yaml, YamlLoader};
use std::collections::HashMap;
use std::path::{PathBuf};
use std::fs::{read_dir, File};
use std::io::prelude::*;

fn main() {
    println!("Starting program...");

    /*
	    1. Загрузить файлы
	    	1.1. Конфиги
	    	1.2. Шаблоны
	    2. Создать структуру админки
	    	2.1. Сервера
	    	2.2. Ядра
	    	2.3. Копии
	    	2.4. Скрипты
	    3. Сгенерировать итоговую структуру на основе шаблонов
	    4. Сохранить в итоговые файлы
	    	4.1. vk (core%, logic%)
	    	4.2. templates.xml

	    # Структуры:
	      - FileManager
	      - Config
	      - TheKingdom
	      - XMLGenerator
	      - FileManagerCommand
    */

    let file_manager = FileManager::new();
    let config = Config::new(file_manager);
    //let key = Yaml::Integer(1);
    println!("{:?}", config.cores[2]);

    //["name"].as_str().unwrap());
    //println!("{:?}", config.test["foo"]);
    //thekingdom = TheKingdom::new(config);
    //xml_generator = XMLGenerator::new();
    //xml_generator.run(thekingdom);
    //xml_generator.save_result(file_manager);

    /*
	py_generator = PYGenerator::new();
	py_generator.run(thekingdom);
	py_generator.save_result();
    */
    //file_manager.save_result_xml(xml_generator.result());
}


static CONFIG_FOLDER: &'static str = "conf";
static TEMPLATES_FOLDER: &'static str = "templates";
static TEMPLATES_XML_SUBFOLDER: &'static str = "xml";
static LOGIC_FOLDER: &'static str = "logic";
static LOGIC_CORES_SUBFOLDER: &'static str = "cores";

static CORES_FILENAME: &'static str = "cores.yaml";
static SERVERS_FILENAME: &'static str = "servers.yaml";
static LOGIC_COMMON_FILENAME: &'static str = "common.yaml";


#[derive(Debug)]
struct Config {
    pub cores: Yaml,
    pub test: Yaml,
    servers: Yaml,
    logic_common: Yaml,
    logic_cores: HashMap<String, Yaml>,
    templates_xml: HashMap<String, String>,
}


impl Config {

    pub fn new(file_manager: FileManager) -> Config {

    	fn convert_to_yaml(file_str: String) -> Yaml {
			let docs = YamlLoader::load_from_str(&file_str).unwrap();
			docs[0].clone()
		}

		fn convert_to_yaml_hashmap(hashmap: HashMap<String, String>) -> HashMap<String, Yaml> {
			let mut result = HashMap::new();
			for (key, value) in hashmap.iter() {
				result.insert(key.clone(), convert_to_yaml(value.to_string()));
			}
			result
		}

		let test_yaml = convert_to_yaml(file_manager.load_config_file(("test.yaml"), None));
    	let cores = convert_to_yaml(file_manager.load_config_file(CORES_FILENAME, None));
    	let servers = convert_to_yaml(file_manager.load_config_file(SERVERS_FILENAME, None));
    	let logic_common = convert_to_yaml(file_manager.load_config_file(LOGIC_COMMON_FILENAME, Some(&vec![LOGIC_FOLDER])));

    	let logic_cores = convert_to_yaml_hashmap(file_manager.load_config_dir(&vec![LOGIC_FOLDER, LOGIC_CORES_SUBFOLDER]));
    	let templates_xml = file_manager.load_config_dir(&vec![TEMPLATES_FOLDER, TEMPLATES_XML_SUBFOLDER]);

    	Config {
    		cores: cores,
    		servers: servers,
    		logic_common: logic_common,
    		logic_cores: logic_cores,
    		templates_xml: templates_xml,
    		test: test_yaml,
    	}
    }
}


#[derive(Debug)]
struct FileManager;

impl FileManager {
    pub fn new() -> FileManager {
    	FileManager
    }

    pub fn load_config_file(&self, file_name: &str, dirs: Option<&Vec<&str>>) -> String {
    	let mut full_path = self.get_config_folder();
    	if let Some(d) = dirs {
    		full_path.extend(d);
    	}
    	self.load_file(Some(file_name), Some(&full_path))
    }

    pub fn load_config_dir(&self, dirs: &Vec<&str>) -> HashMap<String, String> {
    	let mut full_path = self.get_config_folder();
    	full_path.extend(dirs);
    	self.load_dir(&full_path)
    }

    fn get_config_folder(&self) -> Vec<&str> {
    	vec![CONFIG_FOLDER]
    }

    fn load_file(&self, file_name: Option<&str>, dirs: Option<&Vec<&str>>) -> String {
    	let path = self.make_path(dirs, file_name);
    	self.load(path)
    }

    fn load_dir(&self, dirs: &Vec<&str>) -> HashMap<String, String> {
    	let path = self.make_path(Some(dirs), None);
    	let mut result = HashMap::new();
        for entry in read_dir(path.as_path()).unwrap() {
            let entry = entry.unwrap();
            let file_name = entry.file_name().into_string().unwrap();
            result.insert(file_name, self.load(entry.path()));
        }
        result
    }

    fn make_path(&self, dirs: Option<&Vec<&str>>, file_name: Option<&str>) -> PathBuf {
    	let mut path = PathBuf::new();
    	if let Some(d) = dirs {
    		for dir in d {
    			path.push(dir);
    		}
    	}
    	if let Some(f) = file_name {
    		path.push(f);
    	}
    	path
    }

    fn load(&self, path: PathBuf) -> String {
		let mut file = File::open(path).unwrap();
		let mut result = String::new();
		let _ = file.read_to_string(&mut result);
		result
    }

    /*
    pub fn save_result_xml(&self, result: HashMap) {
    	self.save(FileOption::XMLResult, result);
    }*/
}


