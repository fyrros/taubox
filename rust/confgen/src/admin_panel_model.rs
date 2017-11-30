use std::collections::HashMap;

use yaml_rust::Yaml;
use strfmt::strfmt;

use types::*;
use core::Core;
use server::Server;
use config::Config;
//use result::XMLResult;
use logic::*;


static TAB_IN_SERVERS: &'static str = "\n        ";

fn get_id(yaml_id: &Yaml) -> Id {
    yaml_id.as_i64().unwrap() as Id
}


#[derive(Debug)]
struct ServerType<'a> {
    name: &'a str,
    path: &'a str,
}


#[derive(Debug)]
pub struct AdminPanelModel<'a> {
    cores: HashMap<Id,Core>,
    servers: Vec<Server>,
    server_types: [ServerType<'a>; 2],
    logic: LogicManager<'a>,
}

#[allow(unused_variables)]
impl<'a> AdminPanelModel<'a> {

    pub fn new(config: Config) -> AdminPanelModel<'a> {
        let mut thekingdom = AdminPanelModel {
            cores: HashMap::new(),
            servers: Vec::new(),
            server_types: [ServerType{name:"game",path:RESULT_GAME_XML_FILE_PATH},
                           ServerType{name:"logic",path:RESULT_LOGIC_XML_FILE_PATH}],
            logic: LogicManager::new(),
        };

        thekingdom.load_logic(&config);
        thekingdom.load_cores(&config);
        thekingdom.load_servers(&config);
        thekingdom.sort_copies_in_cores();

        thekingdom
    }

    fn load_logic(&mut self, config: &Config) {
        self.logic.load_common(config.get_logic_common());
    }

    fn load_cores(&mut self, config: &Config) {
        for (core_id_yaml, core_config) in config.get_cores() {
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

    fn load_servers(&mut self, config: &Config) {
        for (server_id_yaml, server_config) in config.get_servers() {
            let server_id = get_id(server_id_yaml);
            let mut server = Server::new(server_id, server_config);
            server.add_copies(server_config, &mut self.cores);
            self.servers.push(server);
        }
    }

    fn sort_copies_in_cores(&mut self) {
        for (_,mut core) in &mut self.cores {
            core.sort_copies();
        }
    }

    pub fn get_servers(&self) -> Vec<Server> {
        &self.servers 
    }
}