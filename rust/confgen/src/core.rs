use std::collections::HashMap;

use yaml_rust::Yaml;

use logic::*;
use types::*;


#[derive(Debug)]
pub struct Core {
    id: Id,
    name: String,
    ports: Ports,
    logic: Vec<LogicScript>,
    copies: Vec<CoreCopy>,
    extra_params: Option<String>
}

impl Core {
    
    pub fn new(core_id: Id, core_config: &Yaml, core_logic: Vec<LogicScript>) -> Core {
        Core {
            id: core_id,
            name: core_config["name"].as_str().unwrap().to_string(),
            ports: Ports::new(&core_config["ports"]),
            logic: core_logic,
            copies: Vec::new(),
            extra_params: match core_config["extra_params"].as_str() {
                Some(s) => Some(s.to_string()),
                None => None,
            },
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_port(&self, port_type: &str) -> &Port {
        match port_type {
            "http"  => self.ports.get_http(),
            "game"  => self.ports.get_game(),
            "logic" => self.ports.get_logic(),
            _ => unreachable!(),
        }
    }

    pub fn add_copy(&mut self, copy_num: Id, server_ip: &str) {
        self.copies.push(CoreCopy::new(copy_num, server_ip));
    }

    pub fn get_copies(&self) -> &Vec<CoreCopy> {
        &self.copies
    }

    pub fn get_logic(&self) -> &Vec<LogicScript> {
        &self.logic
    }

    pub fn get_vars(&self) -> HashMap<String, String> {
        let extra_params = match &self.extra_params {
            &Some(ref s) => s.clone(),
            &None => "".to_string(),
        };
        hashmap!{
            "core_id" => self.id,
            "core_name" => self.name,
            "game_port" => self.ports.get_game(),
            "logic_port" => self.ports.get_logic(),
            "http_port" => self.ports.get_http(),
            "extra_params" => extra_params
        }
    }
}


#[derive(Debug)]
struct Ports {
    http: Port,
    game: Port,
    logic: Port,
}

impl Ports {
    
    fn new(ports_info: &Yaml) -> Ports {
        let get_port = |port_name: &str| ports_info[port_name].as_i64().unwrap() as Port;
        Ports {
            http: get_port("http"),
            game: get_port("game"),
            logic: get_port("logic"),
        }
    }

    fn get_http(&self) -> &Port {
        &self.http
    }

    fn get_game(&self) -> &Port {
        &self.game
    }

    fn get_logic(&self) -> &Port {
        &self.logic
    }
}


#[derive(Debug)]
pub struct CoreCopy {
    num: Id,
    server_ip: String,
}

impl CoreCopy {
    fn new(copy_num: Id, server_ip: &str) -> CoreCopy {
        CoreCopy {
            num: copy_num.clone(),
            server_ip: server_ip.to_string(),
        }
    }

    pub fn get_num(&self) -> &Id {
        &self.num
    }

    pub fn get_ip(&self) -> String {
        self.server_ip.clone()
    }
}