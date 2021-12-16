use std::{collections::VecDeque, fs, path::Path, time::Instant};

use itertools::Itertools;

pub fn run() {
    let inputs = read_inputs("inputs/06-input.txt");

    let start = Instant::now();
    solution(&inputs, 80);
    println!(
        "It took {:?} for the first solution to complete.",
        start.elapsed()
    );
    let start = Instant::now();
    solution(&inputs, 256);
    println!(
        "It took {:?} for the second solution to complete.",
        start.elapsed()
    );
}

fn solution(inputs: &VecDeque<usize>, days: u16) {
    let mut input = inputs.clone();

    (0..=days).for_each(|_| {
        input[6] += input[8];
        input.rotate_left(1);
    });

    println!(
        "Total number of lanternfishes is {}.",
        &input.iter().sum::<usize>()
    );
}

fn read_inputs<T: AsRef<Path>>(path: T) -> VecDeque<usize> {
    let file_content = fs::read_to_string(path).expect("Input file not found.");

    let mut inputs: VecDeque<usize> = file_content
        .split(',')
        .counts_by(|c| c.parse::<i8>().unwrap())
        .into_iter()
        .sorted_by_key(|x| x.0)
        .map(|n| n.1)
        .collect();

    inputs.push_front(0); // 0 x 0
    inputs.push_back(0); // 0 x 6
    inputs.push_back(0); // 0 x 7
    inputs.push_back(0); // 0 x 8

    inputs
}
