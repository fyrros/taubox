use std::collections::HashMap;
use std::path::PathBuf;
use std::fs::{read_dir, File};
use std::io::prelude::*;
use std::hash::Hash;
use std::str::FromStr;


pub trait FilePath {
    
    fn get_path(&self, path_str: &str) -> PathBuf {
        PathBuf::from(path_str)
    }
}

pub trait FileSaver : FilePath {

    fn save_file(&self, path_str: &str, body: &str) {
        let path = self.get_path(path_str);
        self.save(&path, body);
    }

    fn save(&self, path: &PathBuf, body: &str) {
        println!("{:?}", path);
        let mut file = File::create(path).unwrap();
        let _ = file.write_all(body.as_bytes());
    }
}

pub trait FileLoader<K:Eq+Hash+FromStr, V> : FilePath {

    fn load_file(&self, path_str: &str) -> V {
        let path = self.get_path(path_str);
        self.open_and_convert(&path)
    }

    fn load_dir(&self, path_str: &str) -> HashMap<K,V> {
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

    fn open_and_convert(&self, path: &PathBuf) -> V {
        let file_str = self.open(path);
        self.convert(file_str)
    }

    fn open(&self, path: &PathBuf) -> String {
        let mut file = File::open(path).unwrap();
        let mut result = String::new();
        let _ = file.read_to_string(&mut result);
        result
    }

    fn convert(&self, file_str: String) -> V;

    fn get_file_key(&self, entry_path: &PathBuf) -> K;
}
