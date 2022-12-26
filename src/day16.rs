use std::fs;
const INPUT_FILE: &str = "data/test16.txt";

pub fn main() -> u32 {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");

    return 0;
}

pub fn main_bonus() {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");
}
