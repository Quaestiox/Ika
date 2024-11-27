use std::fs;

fn read_fs_to_string(path: &str) -> String{
    fs::read_to_string(path).unwrap()

}