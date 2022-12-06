use std::fs;
use itertools::Itertools;
const INPUT_FILE: &str = "data/input06.txt";

fn index_first_marker(s: &str, len_substring: usize) -> usize {
    for i in len_substring..=s.len() {
        // Count the number of unique characters in the substring
        let n = s[(i-len_substring)..i].chars().unique().count();
        if n == len_substring { return i; }
    }
    return 0;
}

pub fn main() -> usize {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");

    return index_first_marker(&contents, 4);
}

pub fn main_bonus() -> usize {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");

    return index_first_marker(&contents, 14);
}