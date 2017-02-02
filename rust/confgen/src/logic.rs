//use std::collections::{BTreeMap};

use yaml_rust::Yaml;


#[derive(Debug)]
pub struct LogicManager<'a> {
    auto_include: Vec<LogicScript>,
    folders_order: Vec<&'a str>,
}

impl<'a> LogicManager<'a> {
    
    pub fn new() -> LogicManager<'a> {
        LogicManager {
            auto_include: Vec::new(),
            folders_order: vec!["quests","social","instances"],
        }
    }

    pub fn load_common(&mut self, config: &Yaml) {
    	let ref special_rules = config["special_rules"];
    	for script_name in config["auto_include"].as_vec().unwrap() {
    		let script = self.new_auto_include_script(&script_name, &special_rules);
            self.auto_include.push(script);
    	}
    }

    fn new_auto_include_script(&mut self, script_name: &Yaml, special_rules: &Yaml) -> LogicScript {
        let mut path = None;
    	if let Some(yaml_string) = special_rules.as_hash().unwrap().get(script_name) {
    		path = yaml_string.as_str();
    	}
        self.new_script(script_name.as_str().unwrap(), path)
    }

    pub fn collect_core_logic(&self, config: &Yaml) -> Vec<LogicScript> {
        let mut core_logic = Vec::new();
        self.add_auto_include(&mut core_logic, &config["exclude"]);        
        self.add_regular_include(&mut core_logic, &config["include"]);
        //TODO: self.add_groups(core_logic, config["groups"]);

        core_logic
    }

    fn add_auto_include(&self, core_logic: &mut Vec<LogicScript>, exclude_yaml: &Yaml) {
        if let Some(exclude_scripts) = exclude_yaml.as_vec() {
            for logic_script in self.auto_include.iter() {
                if !exclude_scripts.contains(&logic_script.get_yaml_name()) {
                    core_logic.push(logic_script.clone());
                }
            }
        } else {
            core_logic.extend(self.auto_include.clone());
        }
    }

    fn add_regular_include(&self, core_logic: &mut Vec<LogicScript>, include_config: &Yaml) {
        for folder_name in self.folders_order.iter() {
            if let Some(folder_scripts) = include_config[*folder_name].as_vec() {
                for yaml_script_name in folder_scripts.iter() {
                    let script_name = yaml_script_name.as_str().unwrap();
                    let script = self.new_script(script_name, format!("{}/{}", *folder_name, script_name));
                    core_logic.push(script)
                }
            }
        }
    }

    fn new_script(&self, script_name: &str, path: Option<&str>) -> LogicScript {
        let mut script = LogicScript::new(script_name);
        if let Some(path_str) = path {
            script.set_path(path_str)
        }
        script
    }
}


#[derive(Debug, Clone)]
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
    		description: String::new(),
    	}
    }

    fn set_path(&mut self, path: &str) {
    	self.path = path.to_string();
    }

    fn get_yaml_name(&self) -> Yaml {
        Yaml::String(self.name.clone())
    }
}
