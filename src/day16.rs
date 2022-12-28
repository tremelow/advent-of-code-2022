use std::fs;
const INPUT_FILE: &str = "data/test17.txt";

pub fn main() {
    let _contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");
}

pub fn main_bonus() {
    let _contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");
}
