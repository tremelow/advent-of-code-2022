#![allow(dead_code)]
mod day01;
mod day02;

fn main() {
    let res = day02::main();
    println!("{res}");
    let res_bonus = day02::main_bonus();
    println!("{res_bonus}");
}
