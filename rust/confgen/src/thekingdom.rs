use std::collections::{HashMap};

use yaml_rust::Yaml;

use types::*;
use core::Core;
use server::Server;
use config::Config;
use logic::LogicCommon;


#[derive(Debug)]
pub struct TheKingdom {
    config: Config,
    cores: HashMap<Id,Core>,
    servers: HashMap<Id,Server>,
    logic_common: LogicCommon,
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
            logic_common: LogicCommon::new(),
        };

        thekingdom.load_logic();
        thekingdom.load_cores();
        thekingdom.load_servers();

        thekingdom
    }

    fn load_logic(&mut self) {
        self.logic_common.load(self.config.get_logic_common());
    }

    fn load_cores(&mut self) {
        for (core_id_yaml, core_config) in self.config.get_cores() {
            let core_id = get_id(core_id_yaml);
            let mut core = Core::new(core_id, core_config);
            let logic_config = self.config.get_logic(&core_id);
            core.add_logic(logic_config, &self.logic_common);
            self.cores.insert(core_id, core);
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

