mod day01;
mod day08;

use std::env;

fn main() {
    let day: i8 = env::args()
        .nth(1)
        .expect("Not a valid day!")
        .parse()
        .expect("Not a valid day!");

    match day {
        1 => day01::run(),
        8 => day08::run(),
        _ => panic!("Either not a valid day or not implemented yet!"),
    }
}
