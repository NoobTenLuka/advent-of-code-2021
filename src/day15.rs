use std::{time::Instant, path::Path, fs};

use itertools::Itertools;

pub fn run() {
    let inputs = read_inputs("inputs/15-input.txt");

    let start = Instant::now();
    solution_1(&inputs);
    println!(
        "It took {:?} for the first solution to complete.",
        start.elapsed()
    );
}

fn solution_1(inputs: &[Vec<i8>]) {
    println!("{:?}", inputs);
}

fn read_inputs<T: AsRef<Path>>(path: T) -> Vec<Vec<i8>> {
    let file_content = fs::read_to_string(path).expect("Input file not found.");

    file_content.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i8).collect_vec()).collect_vec()
}