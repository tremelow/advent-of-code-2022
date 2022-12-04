#![allow(dead_code)]

mod day01;
mod day02;
mod day03;
mod day04;

fn main() {
    let res = day04::main();
    println!("{res}");
    let res_bonus = day04::main_bonus();
    println!("{res_bonus}");
}
