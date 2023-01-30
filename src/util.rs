use std::fs;

pub fn get_input(path: &str) -> String {
    return fs::read_to_string(path).expect("Cannot load file ");
}
