use std::collections::HashMap;

use yaml_rust::Yaml;
use strfmt::strfmt;

use types::*;
use core::Core;
use server::Server;
use config::*;
use logic::*;


static RESULT_TEMPLATES_XML_FILE_PATH: &'static str  = "result/templates.xml";
static RESULT_CORE_XML_FILE_PATH: &'static str       = "result/core{}.xml";
static RESULT_LOGIC_XML_FILE_PATH: &'static str      = "result/logic{}.xml";

static TAB_IN_SERVERS: &'static str = "        ";

fn get_id(yaml_id: &Yaml) -> Id {
    yaml_id.as_i64().unwrap() as Id
}


#[derive(Debug)]
pub struct TheKingdom<'a> {
    config: Config,
    cores: HashMap<Id,Core>,
    servers: HashMap<Id,Server>,
    logic: LogicManager<'a>,
}

#[allow(unused_variables)]
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
            let server_id_vars = hashmap!{"id" => server_id};
            let game_path = strfmt(RESULT_CORE_XML_FILE_PATH, &server_id_vars).unwrap();
            let logic_path = strfmt(RESULT_LOGIC_XML_FILE_PATH, &server_id_vars).unwrap();
            let (game_body, logic_body) = self.generate_server_result(&server_config);
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

    fn generate_server_result(&self, config: &Server) -> (String, String) {
        let game_body = self.generate_gameserver(config);
        let logic_body = self.generate_logicserver(config);

        (game_body, logic_body)
    }

    fn generate_gameserver(&self, server: &Server) -> String {
        let gameservers = self.config.get_template("gameservers");
        let gameserver_copy = self.config.get_template("gameserver_copy");
        let gameserver_include = self.config.get_template("gameserver_include");

        let mut copies: Vec<String> = Vec::new();
        let mut includes: Vec<String> = Vec::new();

        for core_copy in server.get_copies() {
            let copy_vars = core_copy.get_vars(server.get_game_ip());
            copies.push(strfmt(gameserver_copy, &copy_vars).unwrap());
            includes.push(strfmt(gameserver_include, &copy_vars).unwrap());
        }

        let gameservers_vars = hashmap!{
            "copies" => copies.join(TAB_IN_SERVERS),
            "includes" => includes.join(TAB_IN_SERVERS)
        };

        strfmt(gameservers, &gameservers_vars).unwrap()
    }


    fn generate_logicserver(&self, server_config: &Server) -> String {
        let logic_body = String::new();
        let logicservers = self.config.get_template("logicservers");
        let logicserver_copy = self.config.get_template("logicserver_copy");
        let logicserver_include = self.config.get_template("logicserver_include");
        
        logic_body
    }

    fn generate_cores_result(&self) -> String {
        String::new()
    }
}