use std::fs;

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
    let contents = fs::read_to_string("data/input03.txt")
        .expect("Should have been able to read the file.");

    return contents.lines().map(common_item).map(item_value).sum();
}
