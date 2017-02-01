//use std::collections::{BTreeMap};

use yaml_rust::Yaml;


#[derive(Debug)]
pub struct LogicCommon {
    auto_include: Vec<LogicScript>,
}

impl LogicCommon {
    
    pub fn new() -> LogicCommon {
        LogicCommon {
            auto_include: Vec::new(),
        }
    }

    pub fn load(&mut self, config: &Yaml) {
    	let ref special_rules = config["special_rules"];
    	for script_name in config["auto_include"].as_vec().unwrap() {
    		self.add_script(&script_name, &special_rules);
    	}
    }

    fn add_script(&mut self, script_name: &Yaml, special_rules: &Yaml) {
    	let mut logic_script = LogicScript::new(script_name.as_str().unwrap());
    	if let Some(yaml_string) = special_rules.as_hash().unwrap().get(script_name) {
    		let path = yaml_string.as_str().unwrap();
    		logic_script.set_path(path);
    	}
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
