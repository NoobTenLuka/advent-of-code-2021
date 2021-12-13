use std::{path::{Path}, time::Instant, fs, collections::HashSet};

use itertools::Itertools;

#[derive(Debug, PartialEq)]
enum Axis {
    Horizontal,
    Vertical
}

pub fn run() {
    let inputs = read_inputs("inputs/13-input.txt");

    let start = Instant::now();
    solution_1(&inputs);
    println!(
        "It took {:?} for the first solution to complete.",
        start.elapsed()
    );
}

fn solution_1(inputs: &(HashSet<(i32, i32)>, Vec<(Axis, i32)>)) {
    let first_fold = &inputs.1[0];
    let applied_first_fold: HashSet<(i32, i32)> = inputs.0.iter().map(|(mut x, mut y)| {
        match first_fold.0 {
            Axis::Horizontal => {
                if x > first_fold.1 {
                    x -= first_fold.1;
                }
            },
            Axis::Vertical => if y > first_fold.1 {
                y -= first_fold.1;
            },
        }
        (x, y)
    }).collect();

    println!("{:?}", applied_first_fold.len());
}

fn read_inputs<T: AsRef<Path>>(path: T) -> (HashSet<(i32, i32)>, Vec<(Axis, i32)>) {
    let file_content = fs::read_to_string(path).expect("Input file not found.");

    let (points, instructions) = file_content.split_once("\n\n").unwrap();

    let point_set: HashSet<(i32, i32)> = points.lines().map(|l| 
        l.split(',').map(|x| x.parse::<i32>().unwrap()).collect_tuple().unwrap()
    ).collect();

    let instruction_vec = instructions.lines().map(|s| {
        let only_expression = s.chars().skip(11).collect::<String>();
        let (axis_str, num_str) = only_expression.split_once('=').unwrap();

        let axis = if axis_str == "x" {
            Axis::Horizontal
        } else {
            Axis::Vertical
        };

        (axis, num_str.parse::<i32>().unwrap())
    }).collect();

    (point_set, instruction_vec)
}