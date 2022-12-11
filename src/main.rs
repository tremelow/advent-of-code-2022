#![allow(dead_code)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

mod day11;

fn main() {
    let res = day11::main();
    println!("Result of the day: {}", res);
    let res_bonus = day11::main_bonus();
    println!("Bonus result of the day: {}", res_bonus);
}
