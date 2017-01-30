use std::collections::HashMap;
use std::path::PathBuf;
use std::fs::{read_dir, File};
use std::io::prelude::*;

pub trait FileManager<T> {

    fn get_path(&self, path_str: &str) -> PathBuf {
        PathBuf::from(path_str)
    }

    fn load_file(&self, path_str: &str) -> T {
        let path = self.get_path(path_str);
        self.open_and_convert(&path)
    }

    fn load_dir(&self, path_str: &str) -> HashMap<String, T> {
        let path = self.get_path(path_str);
        let mut result = HashMap::new();
        for entry in read_dir(path.as_path()).unwrap() {
            let entry_path = entry.unwrap().path();
            let key = self.get_file_key(&entry_path);
            let value = self.open_and_convert(&entry_path);

            result.insert(key, value);
        }
        result
    }

    fn get_file_key(&self, entry_path: &PathBuf) -> String {
        entry_path.file_stem().unwrap().to_str().unwrap().to_string()
    }

    fn open_and_convert(&self, path: &PathBuf) -> T {
        let file_str = self.open(path);
        self.convert(file_str)
    }

    fn open(&self, path: &PathBuf) -> String {
        let mut file = File::open(path).unwrap();
        let mut result = String::new();
        let _ = file.read_to_string(&mut result);
        result
    }

    fn convert(&self, file_str: String) -> T;
}