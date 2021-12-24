use std::{fs, path::Path, time::Instant};

use itertools::Itertools;

#[derive(Copy, Clone)]
struct Cube {
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
}

pub fn run() {
    let inputs = read_inputs("inputs/22-input.txt");

    let start = Instant::now();
    solution_1(&inputs);
    println!(
        "It took {:?} for the first solution to complete.",
        start.elapsed()
    );
    let start = Instant::now();
    solution_1(&inputs);
    println!(
        "It took {:?} for the second solution to complete.",
        start.elapsed()
    );
}

type CoordinateRange = (i32, i32);

fn solution_1(inputs: &[(bool, (CoordinateRange, CoordinateRange, CoordinateRange))]) {
    println!("{:?}", inputs);
}

fn read_inputs<T: AsRef<Path>>(
    path: T,
) -> Vec<(bool, (CoordinateRange, CoordinateRange, CoordinateRange))> {
    let file_content = fs::read_to_string(path).expect("Input file not found.");

    file_content
        .lines()
        .map(|l| {
            let (status_str, coordinates) = l.split_once(' ').unwrap();

            let status = status_str == "on";

            let comma_split_coords = coordinates.split(',');

            let (x, y, z): ((i32, i32), (i32, i32), (i32, i32)) = comma_split_coords
                .map(|c| {
                    c[2..]
                        .split("..")
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap();

            (status, (x, y, z))
        })
        .collect_vec()
}
