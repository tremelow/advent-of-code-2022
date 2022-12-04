use std::fs;

const INPUT_FILE: &str = "data/input04.txt";
type Range = (i32, i32);

fn str_to_range(s: &str) -> Range {
    let mut r = s.split("-");
    let r1 = r.next().unwrap().parse().unwrap();
    let r2 = r.next().unwrap().parse().unwrap();
    return (r1,r2);
}

fn line_to_ranges(s: &str) -> (Range,Range) {
    let mut ranges = s.split(",");
    return (str_to_range(ranges.next().unwrap()), str_to_range(ranges.next().unwrap()));
}

// Compares ranges r1 and r2
fn does_intersect(r1: Range, r2: Range) -> bool {
    let b = (r1.0 - r2.0) * (r1.1 - r2.1) <= 0;
    return b;
}

pub fn main() -> u32 {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");

    return contents.lines()
        .map(line_to_ranges)
        .map(|r| does_intersect(r.0,r.1))
        .map(|x| x as u32)
        .sum();
}