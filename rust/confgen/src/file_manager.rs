use std::collections::HashMap;
use std::path::{PathBuf};
use std::fs::{read_dir, File};
use std::io::prelude::*;

pub trait FileManager<T> {

    fn convert(&self, file_str: String) -> T;

    fn load(&self, path: PathBuf) -> T {
        let mut file = File::open(path).unwrap();
        let mut result = String::new();
        let _ = file.read_to_string(&mut result);
        self.convert(result)
    }

    fn load_dir(&self, path: PathBuf) -> HashMap<String, T> {
        let mut result = HashMap::new();
        for entry in read_dir(path.as_path()).unwrap() {
            let entry = entry.unwrap();
            let file_name = entry.file_name().into_string().unwrap();
            let conf = self.load(entry.path());

            result.insert(file_name, conf);
        }
        result
    }
}