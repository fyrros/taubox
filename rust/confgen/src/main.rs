extern crate yaml_rust;
extern crate strfmt;

#[macro_use]
mod utils;

mod config;
mod file_manager;
mod thekingdom;
mod logic;
mod core;
mod server;
mod types;

use thekingdom::TheKingdom;


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

    let thekingdom = TheKingdom::new();
    thekingdom.generate();
}

