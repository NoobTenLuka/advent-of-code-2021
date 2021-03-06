use std::{collections::HashMap, fs, path::Path, time::Instant};

use petgraph::graph::{NodeIndex, UnGraph};

pub fn run() {
    let inputs = read_inputs("inputs/12-input.txt");

    let start = Instant::now();
    solution_1(&inputs);
    println!(
        "It took {:?} for the first solution to complete.",
        start.elapsed()
    );
    let start = Instant::now();
    solution_2(&inputs);
    println!(
        "It took {:?} for the second solution to complete.",
        start.elapsed()
    );
}

fn solution_1(inputs: &(UnGraph<(), ()>, HashMap<u32, bool>)) {
    let count = search(inputs, 0, NodeIndex::new(0));

    println!("There are {} paths through the cave system.", &count);
}

fn search(inputs: &(UnGraph<(), ()>, HashMap<u32, bool>), visited: i16, index: NodeIndex) -> i32 {
    if index == NodeIndex::from(1) {
        return 1;
    }

    if !inputs.1.get(&(index.index() as u32)).unwrap()
        && visited & 1 << index.index() == 1 << index.index()
    {
        return 0;
    }

    inputs
        .0
        .neighbors(index)
        .map(|i| search(inputs, visited | 1 << index.index(), i))
        .sum()
}

fn solution_2(inputs: &(UnGraph<(), ()>, HashMap<u32, bool>)) {
    let count = search_2(inputs, 0, NodeIndex::new(0), false);

    println!("There are {} paths through the cave system.", &count);
}

fn search_2(
    inputs: &(UnGraph<(), ()>, HashMap<u32, bool>),
    visited: i16,
    index: NodeIndex,
    mut has_visited_twice: bool,
) -> i32 {
    if index == NodeIndex::from(1) {
        return 1;
    }

    if index == NodeIndex::from(0) && visited & 1 == 1 {
        return 0;
    }

    if !inputs.1.get(&(index.index() as u32)).unwrap()
        && visited & 1 << index.index() == 1 << index.index()
    {
        if has_visited_twice {
            return 0;
        }
        has_visited_twice = true;
    }

    inputs
        .0
        .neighbors(index)
        .map(|i| search_2(inputs, visited | 1 << index.index(), i, has_visited_twice))
        .sum()
}

fn read_inputs<T: AsRef<Path>>(path: T) -> (UnGraph<(), ()>, HashMap<u32, bool>) {
    let file_content = fs::read_to_string(path).expect("Input file not found.");

    let mut current_shift = 2_u32;
    let mut shift_map = HashMap::from([("start", 0), ("end", 1)]);
    let mut uppercase_map = HashMap::from([(0, false), (1, false)]);

    (
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
        })),
        uppercase_map,
    )
}
