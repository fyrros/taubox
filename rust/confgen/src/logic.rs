use yaml_rust::Yaml;
use std::collections::{HashMap, BTreeMap};


#[derive(Debug)]
pub struct LogicManager<'a> {
    auto_include: Vec<LogicScript>,
    folders_order: Vec<&'a str>,
}

impl<'a> LogicManager<'a> {
    
    pub fn new() -> LogicManager<'a> {
        LogicManager {
            auto_include: Vec::new(),
            folders_order: vec!["quests","social","instances","mobs"],
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
    	if let Some(yaml_string) = special_rules[script_name.as_str().unwrap()].as_str() {
    		path = Some(yaml_string.to_string());
    	}
        self.new_script(script_name, path)
    }

    fn new_script(&self, script_name: &Yaml, path: Option<String>) -> LogicScript {
        let mut script = LogicScript::new(script_name.as_str().unwrap());
        if let Some(path_str) = path {
            script.set_path(path_str)
        }
        script
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
            match include_config[*folder_name] {
                Yaml::Array(ref list_dir) => self.add_folder(core_logic, *folder_name, list_dir),
                Yaml::Hash(ref hash_dir) => self.add_tree(core_logic, *folder_name, hash_dir),
                _ => {}
            }
        }
    }

    fn add_folder(&self, core_logic: &mut Vec<LogicScript>, path: &str, config: &Vec<Yaml>) {
        for script_name in config {
            self.add_script(core_logic, &script_name, path);
        }
    }

    fn add_tree(&self, core_logic: &mut Vec<LogicScript>, root_dir: &str, tree: &BTreeMap<Yaml,Yaml>) {
        for (folder_name, folder_scripts) in tree.iter() {
            let path = self.get_path(root_dir, folder_name);
            self.add_folder(core_logic, path.as_str(), folder_scripts.as_vec().unwrap());
        }
    }

    fn add_script(&self, logic: &mut Vec<LogicScript>, script_name: &Yaml, path: &str) {
        let full_path = self.get_path(path, script_name);
        let script = self.new_script(script_name, Some(full_path));
        logic.push(script)
    }

    fn get_path(&self, path_start: &str, yaml_path_end: &Yaml) -> String {
        let path_end = yaml_path_end.as_str().unwrap();
        if path_end != "_" {
            vec![path_start, path_end].join("/")
        } else {
            path_start.to_string()
        }
    }
}


#[derive(Debug, Clone)]
pub struct LogicScript {
    name: String,
    path: String,
}

impl LogicScript {
    
    pub fn new(name: &str) -> LogicScript {
    	LogicScript {
    		name: name.to_string(),
    		path: name.to_string(),
    	}
    }

    fn set_path(&mut self, path: String) {
    	self.path = path;
    }

    fn get_yaml_name(&self) -> Yaml {
        Yaml::String(self.name.clone())
    }

    pub fn add_script_info(&self, core_vars: &mut HashMap<String, String>) {
        let fullname = self.path.replace("/","_").replace("_mob_","_");
        let description = fullname.replace("_"," ");
        core_vars.insert("name".to_string(), self.name.clone());
        core_vars.insert("fullname".to_string(), fullname);
        core_vars.insert("description".to_string(), description);
        core_vars.insert("path".to_string(), self.path.clone());
    }
}
