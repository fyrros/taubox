use std::collections::{HashMap};

use yaml_rust::Yaml;

use types::*;
use core::Core;
use server::Server;
use config::*;
use logic::*;


static RESULT_TEMPLATES_XML_FILE_PATH: &'static str  = "result/templates.xml";
static RESULT_CORE_XML_FILE_PATH: &'static str       = "result/core{}.xml";
static RESULT_LOGIC_XML_FILE_PATH: &'static str      = "result/logic{}.xml";


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
    
    pub fn generate(&self) {
        // servers
        for (server_id, server_config) in self.servers.iter() {
            let game_path = self.format(RESULT_CORE_XML_FILE_PATH.to_string(), "{}", server_id.to_string());
            let logic_path = self.format(RESULT_LOGIC_XML_FILE_PATH.to_string(), "{}", server_id.to_string());
            let (game_body, logic_body) = self.genrate_server_result(&server_config);
            let gameserver_result_file = ConfigFile::new(game_path, game_body);
            let logicserver_result_file = ConfigFile::new(logic_path, logic_body);
            gameserver_result_file.save_config();
            logicserver_result_file.save_config();
        }

        // cores
        let cores_result_path = RESULT_TEMPLATES_XML_FILE_PATH.to_string();
        let cores_result_body = self.generate_cores_result();
        let cores_result_file = ConfigFile::new(cores_result_path, cores_result_body);
        cores_result_file.save_config();
    }

    fn genrate_server_result(&self, config: &Server) -> (String, String) {
        unimplemented!()
    }

    fn generate_cores_result(&self) -> String {
        String::new()
    }

    fn format(&self, template: String, keyword: &str, value: String) -> String {
        template.as_str().replace(keyword, value.as_str())
    }
}