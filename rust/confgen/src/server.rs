use std::collections::{HashMap};

use yaml_rust::Yaml;

use core::Core;
use types::*;


#[derive(Debug)]
pub struct Server {
    id: Id,
    game: ServerInfo,
    logic: ServerInfo,
    copies: Vec<ServerCopyInfo>,
}

impl Server {

    //fn new(server_id: Id, server_config: &Yaml, cores: &'a HashMap<Id,Core>) -> Server<'a> {
    pub fn new(server_id: Id, server_config: &Yaml) -> Server {
        Server {
            id: server_id,
            game: ServerInfo::new(&server_config["game"]),
            logic: ServerInfo::new(&server_config["logic"]),
            copies: Vec::new(),
        }
    }

    pub fn add_copies(&mut self, server_config: &Yaml, cores: &mut HashMap<Id,Core>) {

        fn parse_ids(core_copy_ids_yaml: &Yaml) -> (Id, Id) {
            let copy_full_id: Vec<&str> = core_copy_ids_yaml.as_str().unwrap().split(':').collect();
            let parse_id = |index: usize| copy_full_id[index].parse().unwrap();
            (parse_id(0), parse_id(1))
        }

        for copy_id in server_config["copies"].as_vec().unwrap() {
            let (core_id, copy_num) = parse_ids(copy_id);
            let mut core = cores.get_mut(&core_id).unwrap();
            let core_copy = ServerCopyInfo::new(copy_num, core_id, core.get_name(), core.get_port("http"));
            core.add_copy(copy_num, self.get_game_ip());
            self.copies.push(core_copy);
        };
    }

    fn get_game_ip(&self) -> &str {
        self.game.get_ip()
    }
}

#[derive(Debug)]
struct ServerInfo {
    ident: String,
    ip: String,
}

impl ServerInfo {
    
    fn new(conf: &Yaml) -> ServerInfo {
        let get_string = |key: &str| conf[key].as_str().unwrap().to_string();
        ServerInfo {
            ident: get_string("ident"),
            ip: get_string("ip"),
        }
    }

    fn get_ip(&self) -> &str {
        self.ip.as_str()
    }
}

#[derive(Debug)]
struct ServerCopyInfo {
    num: Id,
    core_id: Id,
    core_name: String,
    http_port: Port,
}

impl ServerCopyInfo {
    
    fn new(copy_num: Id, core_id: Id, core_name: &String, http_port: &Port) -> ServerCopyInfo {
        ServerCopyInfo {
            num: copy_num,
            core_id: core_id,
            core_name: core_name.clone(),
            http_port: http_port.clone(),
        }
    }
}