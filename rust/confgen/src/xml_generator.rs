use types::*;
use file_manager::*;
use yaml_rust::Yaml;
use thekingdom::TheKingdom;



#[derive(Debug)]
pub struct XMLGenerator;

impl XMLGenerator {
    
    pub fn new() -> XMLGenerator {
        XMLGenerator {}
    }

    fn run(&self, model: &AdminPanelModel, templates: &Templates, file_manager: &FileManager) {

    }

    /*
    pub fn generate(&self) {
        // servers
        for (_, server) in self.servers.iter() {
            let result = self.get_result(server);
            result.save();

            let id_vars = hashmap!{"id" => server.get_id()};
            for server_type in self.server_types.iter() {                
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

    fn get_result_file_path(&self, file_type: ResultFileType) -> String {
        match ResultFileType {
            ResultFileType::CoresLogic =>  RESULT_TEMPLATES_XML_FILE_PATH.to_string(),
            ResultFileType::GameServers(i) => expr,
            ResultFileType::LogicServers(i) => 
        }
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
            "server_id" => server.get_id(),
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
        let mut cores_ids = self.cores.keys().collect::<Vec<&Id>>();
        cores_ids.sort();

        for core_id in cores_ids.iter() {
            let core = self.cores.get(core_id).unwrap();
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
                core_vars.insert("scripts".to_string(), scripts.join("\n"));
                groups.push(strfmt(logic_group_template, &core_vars).unwrap());
            }
        }

        let logic_main_template = self.config.get_template("logic_main");
        let logic_main_vars = hashmap!{
            "cores" => cores.join(TAB_IN_SERVERS),
            "logic" => groups.join(TAB_IN_SERVERS)
        };

        strfmt(logic_main_template, &logic_main_vars).unwrap()
    } */

    pub fn process(&self, thekingdom: &TheKingdom) {
        self.process_servers(thekingdom);
        self.process_logic(thekingdom);
    }

    fn process_servers(&self, thekingdom: &TheKingdom) {
        for (_, server) in thekingdom.get_servers().iter() {
            let result = self.get_result(server);
            result.save();
        }
    }
}
