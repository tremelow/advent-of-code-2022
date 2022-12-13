echo 'use std::fs;
const INPUT_FILE: &str = "data/test${1}.txt";

pub fn main() {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");
}

pub fn main_bonus() {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");
}' > src/day${1}.rs 

touch data/test${1}.txt

touch data/input${1}.txt