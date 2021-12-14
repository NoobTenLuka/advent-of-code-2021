use std::{collections::HashMap, fs, path::Path, time::Instant};

use itertools::{Itertools, MinMaxResult};

pub fn run() {
    let inputs = read_inputs("inputs/14-input.txt");

    let start = Instant::now();
    solution_1(&inputs);
    println!(
        "It took {:?} for the first solution to complete.",
        start.elapsed()
    );
}

fn solution_1(inputs: &(String, HashMap<String, String>)) {
    let final_string = (0..10).fold(inputs.0.clone(), |acc, _| {
        let string_vec: Vec<String> = (0..acc.len())
            .map(|i| acc.chars().skip(i).take(2).collect::<String>())
            .collect();

        string_vec
            .iter()
            .map(|e| inputs.1.get(e).unwrap_or(e).clone())
            .collect()
    });

    let counts = final_string.chars().counts();

    if let MinMaxResult::MinMax(min, max) = counts.values().minmax() {
        println!("{}", max - min);
    }
}

fn read_inputs<T: AsRef<Path>>(path: T) -> (String, HashMap<String, String>) {
    let file_content = fs::read_to_string(path).expect("Input file not found.");

    let (input, instructions) = file_content.split_once("\n\n").unwrap();

    let instruction_map = instructions
        .lines()
        .map(|s| {
            let key = s.chars().take(2).collect::<String>();
            let value = s.chars().nth(6).unwrap();

            (key, format!("{}{}", s.chars().nth(0).unwrap(), value))
        })
        .collect();

    (input.to_string(), instruction_map)
}
