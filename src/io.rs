use std::fs;

pub fn read_fs(path: &str) -> String{
    fs::read_to_string(path).unwrap()
}

