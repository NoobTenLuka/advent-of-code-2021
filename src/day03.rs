use std::{fs, path::Path, time::Instant};

pub fn run() {
    let inputs = read_inputs("inputs/03-input.txt");

    let start = Instant::now();
    solution_1(&inputs);
    println!(
        "It took {:?} for the first solution to complete.",
        start.elapsed()
    );
}

fn solution_1(inputs: &[Vec<char>]) {
    let mut counts = [(0, 0); 12];

    inputs.iter().for_each(|i| {
        i.iter().enumerate().for_each(|e| {
            if *e.1 == '0' {
                counts[e.0].0 += 1;
            } else {
                counts[e.0].1 += 1
            }
        })
    });

    let mut gamma_rate = "".to_string();
    let mut epsilon_rate = "".to_string();

    counts.iter().for_each(|c| {
        if c.0 > c.1 {
            gamma_rate.push('0');
            epsilon_rate.push('1');
        } else {
            gamma_rate.push('1');
            epsilon_rate.push('0');
        }
    });

    println!(
        "The product of gamma rate and epsilon rate is {}.",
        isize::from_str_radix(&gamma_rate, 2).unwrap()
            * isize::from_str_radix(&epsilon_rate, 2).unwrap()
    );
}

fn read_inputs<T: AsRef<Path>>(path: T) -> Vec<Vec<char>> {
    let file_content = fs::read_to_string(path).expect("Input file not found.");

    file_content
        .split('\n')
        .map(|l| l.chars().collect())
        .collect()
}
