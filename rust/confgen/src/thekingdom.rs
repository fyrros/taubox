use config::Config;
use yaml_rust::Yaml;
//use std::collections::HashMap;
//use std::collections::HashMap;

#[derive(Debug)]
pub struct TheKingdom {
    cores: Vec<Core>,
    servers: Vec<Server>,
    config: Config,
}

impl TheKingdom {
    pub fn new() -> TheKingdom {
        let mut thekingdom = TheKingdom {
            cores: Vec::new(),
            servers: Vec::new(),
            config: Config::new(),
        };

        thekingdom.init();
        thekingdom
    }

    fn init(&mut self) {
        self.load_cores();
        self.load_servers();
    }

    fn load_cores(&mut self) {}
    fn load_servers(&mut self) {}
}

#[derive(Debug)]
struct ServerInfo;

#[derive(Debug)]
struct Server {
    id: u8,
    game: ServerInfo,
    logic: ServerInfo,
    cores: Vec<Core>,
    copies: Vec<String>,
}

impl Server {
    pub fn new(id: u8, config: &Yaml) -> Server {
        let get_hash = |key: &str| config[key].as_hash().unwrap();

        let game_info = get_hash("game");
        let logic_info = get_hash("logic");

        Server {
            id: id,
            game: ServerInfo,
            logic: ServerInfo,
            cores: Vec::new(),
            copies: Vec::new(),
        }
        /*
        Server {
            id: id,
            game: ServerInfo {
                ip: *game_info.get("ip").unwrap(),
                ident: game_info.get("ident").unwrap(),
            },
            logic: ServerInfo {
                ip: logic_info.get("ip").unwrap(),
                ident: logic_info.get("ident").unwrap(),
            },
            cores: Vec::new(),
            copies: config["copies"].as_vec().unwrap()
        
        }
        */
    }
}

#[derive(Debug)]
struct Core {
    id: u8,
    name: String,
    ports: Ports,
    servers: Vec<Server>,
    logic: Vec<LogicScript>,
    extra_params: String
}

#[derive(Debug)]
struct Ports {
    http: u16,
    game: u16,
    logic: u16,
}

#[derive(Debug)]
struct LogicScript {
    name: String,
    path: String,
    description: String,
}