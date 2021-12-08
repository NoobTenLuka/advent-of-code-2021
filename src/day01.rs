use std::{fs, path::Path, time::Instant};

pub fn run() {
    let inputs = read_inputs("inputs/01-input.txt");

    let start = Instant::now();
    solution_1(&inputs);
    println!(
        "It took {:?} for the first solution to complete.",
        start.elapsed()
    );
    let start = Instant::now();
    solution_2(&inputs);
    println!(
        "It took {:?} for the first solution to complete.",
        start.elapsed()
    );
}

fn solution_1(inputs: &[i16]) {
    let mut increased = 0;

    inputs.iter().reduce(|last, new| {
        if new > last {
            increased += 1;
        }
        new
    });

    println!("The level has increased {} times.", increased);
}

fn solution_2(inputs: &[i16]) {
    let new_arr: Vec<i16> = (0..inputs.len() - 2)
        .map(|i| inputs.iter().skip(i).take(3).sum())
        .collect();

    let mut increased = 0;

    new_arr.iter().reduce(|last, new| {
        if new > last {
            increased += 1;
        }
        new
    });

    println!("The windows of three have increased {} times.", increased);
}

fn read_inputs<T: AsRef<Path>>(path: T) -> Vec<i16> {
    let file_contents = fs::read_to_string(path).expect("Input file not found.");

    file_contents
        .split('\n')
        .map(|n| n.parse::<i16>().unwrap())
        .collect()
}
