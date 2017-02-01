use std::collections::{HashMap};

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
    
    pub fn new(core_id: Id, core_config: &Yaml) -> Core {
        Core {
            id: core_id,
            name: core_config["name"].as_str().unwrap().to_string(),
            ports: Ports::new(&core_config["ports"]),
            logic: Vec::new(),
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

    pub fn add_logic(&mut self, logic: &Yaml, logic_common: &LogicCommon) {

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
struct CoreCopy {
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
}

