use std::collections::{HashMap};

use yaml_rust::Yaml;

use config::Config;
use server::Server;
use core::Core;
use types::*;


#[derive(Debug)]
pub struct TheKingdom {
    config: Config,
    cores: HashMap<Id,Core>,
    servers: HashMap<Id,Server>,
}


fn get_id(yaml_id: &Yaml) -> Id {
    yaml_id.as_i64().unwrap() as Id
}


impl TheKingdom {

    pub fn new() -> TheKingdom {
        let mut thekingdom = TheKingdom {
            config: Config::new(),
            cores: HashMap::new(),
            servers: HashMap::new(),
        };

        thekingdom.load_cores();
        thekingdom.load_servers();
        thekingdom
    }

    fn load_cores(&mut self) {
        for (core_id_yaml, core_config) in self.config.get_cores() {
            let core_id = get_id(core_id_yaml);
            self.cores.insert(core_id, Core::new(core_id, core_config));
        }
    }

    fn load_servers(&mut self) {
        for (server_id_yaml, server_config) in self.config.get_servers() {
            let server_id = get_id(server_id_yaml);
            let mut server = Server::new(server_id, server_config);
            server.add_copies(server_config, &mut self.cores);
            self.servers.insert(server_id, server);
        }
    }
    
    pub fn generate_configs(&self) {

    }
}