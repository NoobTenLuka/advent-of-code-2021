use std::{cell::Cell, collections::HashMap, fs, path::Path, rc::Rc, time::Instant};

use petgraph::graph::{NodeIndex, UnGraph};

pub fn run() {
    let inputs = read_inputs("inputs/12-input.txt");

    let start = Instant::now();
    solution_1(&inputs);
    println!(
        "It took {:?} for the first solution to complete.",
        start.elapsed()
    );
}

fn solution_1(inputs: &(UnGraph<(), ()>, HashMap<u32, bool>)) {
    let count = Rc::new(Cell::new(0));

    search(inputs, 0, &count, NodeIndex::new(1));

    println!("{:?}", count);
}

fn search(
    inputs: &(UnGraph<(), ()>, HashMap<u32, bool>),
    visited: i16,
    count_ref: &Rc<Cell<i32>>,
    index: NodeIndex,
) {
    if index == NodeIndex::from(2) {
        count_ref.set(count_ref.get() + 1);
        return;
    }

    if !inputs.1.get(&(index.index() as u32)).unwrap()
        && visited & 1 << index.index() == 1 << index.index()
    {
        return;
    }

    inputs
        .0
        .neighbors(index)
        .for_each(|i| search(inputs, visited | 1 << index.index(), count_ref, i));
}

fn read_inputs<T: AsRef<Path>>(path: T) -> (UnGraph<(), ()>, HashMap<u32, bool>) {
    let file_content = fs::read_to_string(path).expect("Input file not found.");

    let mut current_shift = 3_u32;
    let mut shift_map = HashMap::from([("start", 1), ("end", 2)]);
    let mut uppercase_map = HashMap::from([(1, false), (2, false)]);

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
