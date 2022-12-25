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


fn aux_snafu_to_decimal(dec: i32, snafu: &mut Chars) -> i32 {
    if let Some(digit) = snafu.next() {
        aux_snafu_to_decimal(5*dec + VAL[&digit], snafu)
    } else {
        dec
    }
}
fn snafu_to_decimal(snafu: &str) -> i32 {
    let mut c = snafu.chars();
    return aux_snafu_to_decimal(0, &mut c);
}

fn aux_decimal_to_snafu(n: i32, snafu: &mut String) {
    if n == 0 {
        return ;
    }
    let i = ((n + 2) as usize) % 5;
    let d = ((n+2) % 5) - 2;
    let c = SNAFU[i];
    snafu.push(c);
    aux_decimal_to_snafu((n-d)/5, snafu);
}
fn decimal_to_snafu(n: i32) -> String {
    let mut snafu = String::new();
    aux_decimal_to_snafu(n, &mut snafu);
    if snafu.is_empty() {
        snafu.push('0');
    }
    // reverse the string
    return snafu.chars().rev().fold(String::new(), |mut s, c| {s.push(c); s});
}

pub fn main() -> String {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");
    // for l in contents.lines() {
    //     println!("{}", snafu_to_decimal(l));
    // }
    // let s1 = Snafu::from("1==");
    // let s2 = Snafu::from("1=");
    let mut res = Snafu::new();
    let mut test = 0;
    // for l in contents.lines() {
    //     res = res + Snafu::from(l);
    //     test = test + snafu_to_decimal(l);
    //     println!("Reference method: {}", decimal_to_snafu(test));
    //     println!("Tested method:    {}", res.0);
    //     println!();
    // }
    
    let res = contents.lines().map(Snafu::from).fold(Snafu::new(),|a,b| a+b);

    return res.0;
}

pub fn main_bonus() {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");
}
