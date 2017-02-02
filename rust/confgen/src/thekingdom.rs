use std::collections::{HashMap};

use yaml_rust::Yaml;

use types::*;
use core::Core;
use server::Server;
use config::Config;
use logic::*;


#[derive(Debug)]
pub struct TheKingdom<'a> {
    config: Config,
    cores: HashMap<Id,Core>,
    servers: HashMap<Id,Server>,
    logic: LogicManager<'a>,
}


fn get_id(yaml_id: &Yaml) -> Id {
    yaml_id.as_i64().unwrap() as Id
}


impl<'a> TheKingdom<'a> {

    pub fn new() -> TheKingdom<'a> {
        let mut thekingdom = TheKingdom {
            config: Config::new(),
            cores: HashMap::new(),
            servers: HashMap::new(),
            logic: LogicManager::new(),
        };

        thekingdom.load_logic();
        thekingdom.load_cores();
        thekingdom.load_servers();

        thekingdom
    }

    fn load_logic(&mut self) {
        self.logic.load_common(self.config.get_logic_common());
    }

    fn load_cores(&mut self) {
        for (core_id_yaml, core_config) in self.config.get_cores() {
            let core_id = get_id(core_id_yaml);
            let core_logic = self.get_core_logic(&core_id);
            let core = Core::new(core_id, core_config, core_logic);
            self.cores.insert(core_id, core);
        }
    }

    fn get_core_logic(&self, core_id: &Id) -> Vec<LogicScript> {
        let logic_config = self.config.get_core_logic(&core_id);
        self.logic.collect_core_logic(logic_config)
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

