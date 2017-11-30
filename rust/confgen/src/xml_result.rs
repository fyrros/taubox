use file_manager::*;
use types::*;

static RESULT_TEMPLATES_XML_FILE_PATH: &'static str  = "result/templates.xml";
static RESULT_GAME_XML_FILE_PATH: &'static str       = "result/core{id}.xml";
static RESULT_LOGIC_XML_FILE_PATH: &'static str      = "result/logic{id}.xml";


pub struct XMLResult {

}


#[derive(Debug)]
enum ResultFileType {
    CoresLogic,
    GameServers(Id),
    LogicServers(Id),
}


#[derive(Debug)]
struct ResultFile {
    path: String,
    body: String,
}

impl ResultFile {
    
    pub fn new(path_string: String, body: String) -> ResultFile {
        ResultFile {
            path: path_string,
            body: body,
        }
    }

    pub fn save_config(&self) {
        self.save_file(self.path.as_str(), self.body.as_str());
    }
}

impl FilePath for ResultFile{}
impl FileSaver for ResultFile{}