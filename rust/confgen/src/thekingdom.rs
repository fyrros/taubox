use config::Config;
use yaml_rust::Yaml;
use std::collections::{HashMap, BTreeMap};


#[derive(Debug)]
pub struct TheKingdom<'a> {
    cores: HashMap<Id,Core<'a>>,
    servers: HashMap<Id,Server<'a>>,
    config: Config,
}

type Id = u8;

fn get_id(yaml_id: &Yaml) -> Id {
    yaml_id.as_i64().unwrap() as Id
}


impl<'a> TheKingdom<'a> {

    pub fn new() -> TheKingdom<'a> {
        let mut thekingdom = TheKingdom {
            cores: HashMap::new(),
            servers: HashMap::new(),
            config: Config::new(),
        };

        thekingdom.load_cores();
        thekingdom.load_servers();
        thekingdom.load_logic();
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
            let server = Server::new(server_id, server_config, &mut self.cores);
            self.servers.insert(server_id, server);
        }
    }

    fn load_logic(&mut self) {

    }
    
    pub fn generate_configs(&self) {}
}

#[derive(Debug)]
struct Core<'a> {
    id: Id,
    name: String,
    ports: Ports,
    logic: Vec<LogicScript>,
    servers: HashMap<Id,Server<'a>>,    
    extra_params: Option<String>
}

impl<'a> Core<'a> {
    
    fn new(core_id: Id, core_config: &Yaml) -> Core<'a> {
        Core {
            id: core_id,
            name: core_config["name"].as_str().unwrap().to_string(),
            ports: Ports::new(&core_config["ports"]),
            logic: Vec::new(),
            servers: HashMap::new(),
            extra_params: match core_config["extra_params"].as_str() {
                Some(s) => Some(s.to_string()),
                None => None,
            },
        }
    }
}

#[derive(Debug)]
struct Ports {
    http: u16,
    game: u16,
    logic: u16,
}

impl Ports {
    
    fn new(ports_info: &Yaml) -> Ports {
        let get_port = |port_name: &str| ports_info[port_name].as_i64().unwrap() as u16;
        Ports {
            http: get_port("http"),
            game: get_port("game"),
            logic: get_port("logic"),
        }
    } 
}

#[derive(Debug)]
struct LogicScript {
    name: String,
    path: String,
    description: String,
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
}

#[derive(Debug)]
struct Server<'a> {
    id: Id,
    game: ServerInfo,
    logic: ServerInfo,
    cores: HashMap<Id,&'a Core<'a>>,
    copies: Vec<CoreCopy>,
}

impl<'a> Server<'a> {

    fn new(server_id: Id, server_config: &Yaml, cores: &'a mut HashMap<Id,Core>) -> Server<'a> {
        let mut copies = Vec::new();
        let mut server_cores: HashMap<Id, &Core> = HashMap::new();
        
        for copy_id in server_config["copies"].as_vec().unwrap() {
            let core_copy = CoreCopy::new(&copy_id);
            {
                if let Some(core) = cores.get(&core_copy.core_id) {
                    server_cores.insert(core_copy.core_id, core);
                } else {
                    println!("Error! {:?} not found for {:?}", core_copy, server_id);
                }
            }
            copies.push(core_copy);
        }

        Server {
            id: server_id,
            game: ServerInfo::new(&server_config["game"]),
            logic: ServerInfo::new(&server_config["logic"]),
            cores: server_cores,
            copies: copies,
        }
    }
}

#[derive(Debug)]
struct CoreCopy {
    core_id: Id,
    copy_id: Id,
}

impl CoreCopy {
    
    fn new(copy_full_id_yaml: &Yaml) -> CoreCopy {
        let copy_full_id: Vec<&str> = copy_full_id_yaml.as_str().unwrap().split(':').collect();
        let parse_id = |index: usize| copy_full_id[index].parse().unwrap();
        CoreCopy {
            core_id: parse_id(0),
            copy_id: parse_id(1),
        }
    }
}