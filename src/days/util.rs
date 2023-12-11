use std::fs;

pub fn load_input(path: &str) -> String {
    fs::read_to_string(path).expect(format!("Could not open file {:?}.", path).as_str())
}
