#[derive(Debug)]
struct Templates {
    xml: HashMap<String, Yaml>
}

static TEMPLATES_FILE_PATH: &'static str = "conf/templates.yaml";

fn new() -> Templates {
    Templates {
        xml: HashMap::new()
    }
}

fn configure(&mut self, file_manager: &FileManager) {
    let templates_yaml = file_manager.load_file(TEMPLATES_FILE_PATH);

    for (template_name, template_yaml) in templates_yaml.as_hash().unwrap() {
        self.xml.insert(template_name.as_str().unwrap().to_string(), template_yaml);
    }    
}