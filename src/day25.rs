use std::fs;
const INPUT_FILE: &str = "data/input25.txt";

use std::str::Chars;
use std::iter::Rev;
use std::ops::Add;
use phf::phf_map;

const VAL: phf::Map<char,i32> = phf_map! {
    '2' =>  2,
    '1' =>  1,
    '0' =>  0,
    '-' => -1,
    '=' => -2,
};
const SNAFU: [char;5] = ['=','-','0','1','2'];

struct Snafu(String);
impl Snafu {
    fn new() -> Self {
        Snafu(String::new())
    }
    fn from(s: &str) -> Self {
        Snafu(String::from(s))
    }
    fn iter(&self) -> IterSnafu {
        IterSnafu(self.0.chars().rev())
    }
}
impl Add for Snafu {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let sum = aux_add_snafu(&mut self.iter(), &mut rhs.iter(), 0);
        Snafu::from(sum.trim_start_matches('0'))
    }
}

struct IterSnafu<'a>(Rev<Chars<'a>>);
impl<'a> Iterator for IterSnafu<'a> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
fn aux_add_snafu<'a>(lhs: &mut IterSnafu, rhs: &mut IterSnafu, offset: i32) -> String {
    let (s1, s2) = (lhs.next(), rhs.next());
    if (s1,s2) == (None,None) {
        return String::from(SNAFU[(offset+2) as usize]);
    }

    let val_sum = VAL[&s1.unwrap_or('0')] + VAL[&s2.unwrap_or('0')] + offset;
    let idx_first_digit = (val_sum + 2).rem_euclid(5) as usize;
    let first_digit = (val_sum + 2).rem_euclid(5) - 2;
    let offset = (val_sum - first_digit)/5;
    // println!("New digit: {} (sum of digits was {}), with carried-on offset {}", first_digit, val_sum, offset);

    let mut rest = aux_add_snafu(lhs, rhs, offset);
    rest.push(SNAFU[idx_first_digit]);
    return rest;
}

pub fn main() -> String {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");

    let res = contents.lines().map(Snafu::from).fold(Snafu::new(),|a,b| a+b);
    return res.0;
}

// pub fn main_bonus() {
//     let contents = fs::read_to_string(INPUT_FILE)
//         .expect("Should have been able to read the file.");
// }
