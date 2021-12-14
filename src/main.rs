mod day01;
mod day02;
mod day03;
mod day06;
mod day08;
mod day10;
mod day12;
mod day13;
mod day14;

use std::env;

fn main() {
    let day: i8 = env::args()
        .nth(1)
        .expect("Not a valid day!")
        .parse()
        .expect("Not a valid day!");

    match day {
        1 => day01::run(),
        2 => day02::run(),
        3 => day03::run(),
        6 => day06::run(),
        8 => day08::run(),
        10 => day10::run(),
        12 => day12::run(),
        13 => day13::run(),
        14 => day14::run(),
        _ => panic!("Either not a valid day or not implemented yet!"),
    };
}
