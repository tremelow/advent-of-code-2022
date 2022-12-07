#![allow(dead_code)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

fn main() {
    let res = day07::main();
    println!("{}", res);
    let res_bonus = day07::main_bonus();
    println!("{}", res_bonus);
}
