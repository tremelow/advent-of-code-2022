use std::fs;

const INPUT_FILE: &str = "data/input03.txt";

fn common_item(rucksack: &str) -> char {
    let separating_index = rucksack.len() / 2;

    let compartment1 = &rucksack[..separating_index];
    let compartment2 = &rucksack[separating_index..];

    // Find common item
    for item1 in compartment1.chars() {
        for item2 in compartment2.chars() {
            if item1 == item2 {return item1};
        };
    };

    return ' ';
}

fn item_value(item: char) -> u32 {
    if item.is_uppercase() {
        return item.to_digit(36).unwrap() + 26 - 9;
    } else {
        return item.to_digit(36).unwrap() - 9;
    }
}

pub fn main() -> u32 {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");

    return contents.lines().map(common_item).map(item_value).sum();
}


fn find_first_common(s1: &str, s2: &str, s3: &str) -> char {
    for c1 in s1.chars() {
        for c2 in s2.chars() {
            if c1 == c2 {
                for c3 in s3.chars() {
                    if c1 == c3 {return c1;}
                }
            }
        }
    }
    return ' ';
}

pub fn main_bonus() -> u32 {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");

    let contents: Vec<&str> = contents.lines().collect();

    return contents.chunks_exact(3)
        .map(|s| find_first_common(s[0],s[1],s[2]))
        .map(item_value)
        .sum();
}