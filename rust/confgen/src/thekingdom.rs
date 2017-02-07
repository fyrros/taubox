use std::collections::HashMap;

use yaml_rust::Yaml;
use strfmt::strfmt;

use types::*;
use core::Core;
use server::Server;
use config::*;
use logic::*;


static RESULT_TEMPLATES_XML_FILE_PATH: &'static str  = "result/templates.xml";
static RESULT_GAME_XML_FILE_PATH: &'static str       = "result/core{id}.xml";
static RESULT_LOGIC_XML_FILE_PATH: &'static str      = "result/logic{id}.xml";

static TAB_IN_SERVERS: &'static str = "        ";

fn get_id(yaml_id: &Yaml) -> Id {
    yaml_id.as_i64().unwrap() as Id
}

#[derive(Debug)]
struct ServerType<'a> {
    name: &'a str,
    path: &'a str,
}


#[derive(Debug)]
pub struct TheKingdom<'a> {
    config: Config,
    cores: HashMap<Id,Core>,
    servers: HashMap<Id,Server>,
    server_types: [ServerType<'a>; 2],
    logic: LogicManager<'a>,
}

#[allow(unused_variables)]
impl<'a> TheKingdom<'a> {

    pub fn new() -> TheKingdom<'a> {
        let mut thekingdom = TheKingdom {
            config: Config::new(),
            cores: HashMap::new(),
            servers: HashMap::new(),
            server_types: [ServerType{name:"game",path:RESULT_GAME_XML_FILE_PATH},
                           ServerType{name:"logic",path:RESULT_LOGIC_XML_FILE_PATH}],
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
        for (server_id, server) in self.servers.iter() {
            for server_type in self.server_types.iter() {
                let id_vars = hashmap!{"id" => server.get_id()};
                let path = strfmt(server_type.path, &id_vars).unwrap();
                let body = self.generate_servers_body(server_type, server);
                let result_file = ResultFile::new(path, body);
                result_file.save_config();
            }
        }

        // cores
        let cores_result_path = RESULT_TEMPLATES_XML_FILE_PATH.to_string();
        let cores_result_body = self.generate_cores_body();
        let cores_result_file = ResultFile::new(cores_result_path, cores_result_body);
        cores_result_file.save_config();
    }

    fn generate_servers_body(&self, server_type: &ServerType, server: &Server) -> String {      
        let mut copies = Vec::new();
        let mut includes = Vec::new();

        let copy_template = self.config.get_template(format!("{}server_copy", server_type.name).as_str());
        let include_template = self.config.get_template(format!("{}server_include", server_type.name).as_str());
        let server_ip = match server_type.name {
            "game" => server.get_game_ip(),
            "logic" => server.get_logic_ip(),
            _ => unreachable!(),
        };
        let server_ident = match server_type.name {
            "game" => server.get_game_ident(),
            "logic" => server.get_logic_ident(),
            _ => unreachable!(),
        };

        for core_copy in server.get_copies() {
            let copy_vars = core_copy.get_vars(server_ip);
            copies.push(strfmt(copy_template, &copy_vars).unwrap());
            includes.push(strfmt(include_template, &copy_vars).unwrap());
        }

        let servers_template = self.config.get_template(format!("{}servers", server_type.name).as_str());
        let servers_vars = hashmap!{
            "ident" => server_ident,
            "copies" => copies.join(TAB_IN_SERVERS),
            "includes" => includes.join(TAB_IN_SERVERS)
        };

        strfmt(servers_template, &servers_vars).unwrap()
    }

    fn generate_cores_body(&self) -> String {
        let mut cores = Vec::new();
        let mut groups = Vec::new();

        let core_name_comment_template = self.config.get_template("core_name_comment");
        let core_service_template = self.config.get_template("core_service");
        let logic_group_template = self.config.get_template("logic_group");
        let logic_service_template = self.config.get_template("logic_service");

        for (core_id, core) in self.cores.iter() {
            let mut core_vars = core.get_vars();
            cores.push(strfmt(core_name_comment_template, &core_vars).unwrap());
            for copy in core.get_copies() {
                core_vars.insert("copy_num".to_string(), copy.get_num().to_string());
                core_vars.insert("server_ip".to_string(), copy.get_ip());
                cores.push(strfmt(core_service_template, &core_vars).unwrap());
                
                let mut scripts = Vec::new();
                for script in core.get_logic() {
                    script.add_script_info(&mut core_vars);
                    scripts.push(strfmt(logic_service_template, &core_vars).unwrap());
                }
                core_vars.insert("scripts".to_string(), scripts.join(TAB_IN_SERVERS));
                groups.push(strfmt(logic_group_template, &core_vars).unwrap());
            }
        }

        let logic_main_template = self.config.get_template("logic_main");
        let logic_main_vars = hashmap!{
            "cores" => cores.join(TAB_IN_SERVERS),
            "logic" => groups.join(TAB_IN_SERVERS)
        };

        strfmt(logic_main_template, &logic_main_vars).unwrap()
    }
}