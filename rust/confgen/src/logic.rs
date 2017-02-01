//use std::collections::{BTreeMap};

use yaml_rust::Yaml;


#[derive(Debug)]
pub struct LogicManager {
    auto_include: Vec<LogicScript>,
}

impl LogicManager {
    
    pub fn new() -> LogicManager {
        LogicManager {
            auto_include: Vec::new(),
        }
    }

    pub fn load_common(&mut self, config: &Yaml) {
    	let ref special_rules = config["special_rules"];
    	for script_name in config["auto_include"].as_vec().unwrap() {
    		let script = self.new_script(&script_name, &special_rules);
            self.auto_include.push(script);
    	}
    }

    fn new_script(&mut self, script_name: &Yaml, special_rules: &Yaml) -> LogicScript {
    	let mut logic_script = LogicScript::new(script_name.as_str().unwrap());
    	if let Some(yaml_string) = special_rules.as_hash().unwrap().get(script_name) {
    		let path = yaml_string.as_str().unwrap();
    		logic_script.set_path(path);
    	}
        logic_script
    }

    pub fn collect_core_logic(&self, config: &Yaml) -> Vec<LogicScript> {
        
    }
}


#[derive(Debug)]
pub struct LogicScript {
    name: String,
    path: String,
    description: String,
}

impl LogicScript {
    
    pub fn new(name: &str) -> LogicScript {
    	LogicScript {
    		name: name.to_string(),
    		path: name.to_string(),
    		description: String::new()
    	}
    }

    fn set_path(&mut self, path: &str) {
    	self.path = path.to_string();
    }
}
