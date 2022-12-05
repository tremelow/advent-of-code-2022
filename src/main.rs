#![allow(dead_code)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() {
    let res = day05::main();
    println!("{}", res);
    let res_bonus = day05::main_bonus();
    println!("{res_bonus}");
}
