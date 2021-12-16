use std::{collections::HashMap, fs, path::Path, time::Instant};

use itertools::{Itertools, MinMaxResult};

pub fn run() {
    let inputs = read_inputs("inputs/14-input.txt");

    let start = Instant::now();
    solution(&inputs, 10);
    println!(
        "It took {:?} for the first solution to complete.",
        start.elapsed()
    );
    let start = Instant::now();
    solution(&inputs, 40);
    println!(
        "It took {:?} for the second solution to complete.",
        start.elapsed()
    );
}

fn solution(inputs: &(String, HashMap<String, (String, String)>), days: i8) {
    let pair_counts = (0..inputs.0.len() - 1).map(|i| inputs.0.chars().skip(i).take(2).collect::<String>()).counts();

    let final_pair_counts = (0..days).fold(pair_counts, |acc, _| {
        let mut map = HashMap::new();
        acc.iter().for_each(|(k, v)| {
            let (first, second) = match inputs.1.get(k) {
                Some(x) => x,
                None => return,
            };
            let first_size = *map.get(first).unwrap_or(&0);
            map.insert(first.to_string(), first_size + *v);
            let second_size = *map.get(second).unwrap_or(&0);
            map.insert(second.to_string(), second_size + *v);
        });
        map
    });

    let mut final_counts: HashMap<char, usize> = final_pair_counts.iter().map(|(k, v)| {
        (k.chars().next().unwrap(), *v)
    }).into_group_map().iter().map(|m| (*m.0, (*m.1).iter().sum())).collect();

    *final_counts.get_mut(&inputs.0.chars().last().unwrap()).unwrap() += 1;

    if let MinMaxResult::MinMax(min, max) = final_counts.iter().map(|(_, v)| v).minmax() {
        println!("The result is {}", max - min);
    }
}

fn read_inputs<T: AsRef<Path>>(path: T) -> (String, HashMap<String, (String, String)>) {
    let file_content = fs::read_to_string(path).expect("Input file not found.");

    let (input, instructions) = file_content.split_once("\n\n").unwrap();

    let instruction_map = instructions
        .lines()
        .map(|s| {
            let key = s.chars().take(2).collect::<String>();
            let value = s.chars().nth(6).unwrap();

            (key, (
                format!("{}{}", s.chars().next().unwrap(), value),
                format!("{}{}", value, s.chars().nth(1).unwrap())
            ))
        })
        .collect();

    (input.to_string(), instruction_map)
}
