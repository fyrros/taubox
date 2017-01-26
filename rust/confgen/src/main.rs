fn main() {
    println!("Starting program...");

    /*
	    1. Загрузить файлы
	    	1.1. Конфиги
	    	1.2. Шаблоны
	    2. Создать структуру админки
	    	2.1. Сервера
	    	2.2. Ядра
	    	2.3. Копии
	    	2.4. Скрипты
	    3. Сгенерировать итоговую структуру на основе шаблонов
	    4. Сохранить в итоговые файлы
	    	4.1. vk (core%, logic%)
	    	4.2. templates.xml

	    # Структуры:
	      - FileManager
	      - Config
	      - TheKingdom
	      - XMLGenerator
	      - FileManagerCommand
    */

    file_manager = FileManager::new();
    config = Config::new(file_manager.load_configs());
    dummies = Dummies::new(file_manager.load_dummies());
    thekingdom = TheKingdom::new(config, dummies);
    xml_generator = XMLGenerator::new();
    xml_generator.run(thekingdom);
    file_manager.save_xml_result(xml_generator.result());
}


#[derive(Debug)]
enum FileType {
    Config,
    Dummies,
    XMLResult,
}


#[derive(Debug)]
struct FileManager;

impl FileManager {
    fn new() -> FileManager {
    	FileManager {}
    }

    fn load_configs(&self) -> HashMap {
    	//self.load(FileOption::Config)
    }

    fn load_dummies(&self) -> HashMap {

    }

    fn load(&self, file_option: FileOption) -> T {
    	match file_option {
    	    FileOption::Config => create_config_from_files(),
    	    _ => println!("ERROR! Wrong file option.");,
    	}
    }

    fn save_xml_result(&self, result: HashMap) {
    	self.save(FileOption::XMLResult, result);
    }
}


#[derive(Debug)]
struct Config {
    cores: yaml::Yaml,
    servers: yaml::Yaml,
    cores_logic: Vec<yaml::Yaml>,
    common_logic: yaml::Yaml,
    dummies: HashMap,
}
