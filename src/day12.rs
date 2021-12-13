use std::{collections::HashMap, fs, path::Path, time::Instant};

use petgraph::graph::UnGraph;

pub fn run() {
    let inputs = read_inputs("inputs/12-input.txt");

    let start = Instant::now();
    solution_1(&inputs);
    println!(
        "It took {:?} for the first solution to complete.",
        start.elapsed()
    );
}

fn solution_1(inputs: &UnGraph<(), ()>) {
    println!("{:?}", inputs);
}

fn read_inputs<T: AsRef<Path>>(path: T) -> UnGraph<(), ()> {
    let file_content = fs::read_to_string(path).expect("Input file not found.");

    let mut current_shift = 3;
    let mut shift_map = HashMap::from([("start", 1), ("end", 2)]);
    let mut uppercase_map = HashMap::from([(2, false), (2, false)]);

    UnGraph::from_edges(file_content.lines().map(|l| {
        let (from_str, to_str) = l.split_once('-').unwrap();

        let from = match shift_map.get(from_str) {
            Some(x) => *x,
            None => {
                shift_map.insert(from_str, current_shift);
                current_shift += 1;
                current_shift - 1
            }
        };

        uppercase_map.insert(from, from_str.chars().next().unwrap().is_uppercase());

        let to = match shift_map.get(to_str) {
            Some(x) => *x,
            None => {
                shift_map.insert(to_str, current_shift);
                current_shift += 1;
                current_shift - 1
            }
        };

        uppercase_map.insert(to, to_str.chars().next().unwrap().is_uppercase());

        (from, to)
    }))
}
