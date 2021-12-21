use std::{collections::HashSet, fs, path::Path, time::Instant};

use itertools::Itertools;

#[derive(Debug, PartialEq)]
enum Axis {
    Horizontal,
    Vertical,
}

type FoldingInstructions = Vec<(Axis, i32)>;
type CoordinateSet = HashSet<(i32, i32)>;

pub fn run() {
    let inputs = read_inputs("inputs/13-input.txt");

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

fn solution_1(inputs: &(CoordinateSet, FoldingInstructions)) {
    let first_fold = &inputs.1[0];
    let applied_first_fold: HashSet<(i32, i32)> = inputs
        .0
        .iter()
        .map(|(mut x, mut y)| {
            match first_fold.0 {
                Axis::Horizontal => {
                    if x > first_fold.1 {
                        x += (first_fold.1 - x) * 2;
                    }
                }
                Axis::Vertical => {
                    if y > first_fold.1 {
                        y += (first_fold.1 - y) * 2;
                    }
                }
            }
            (x, y)
        })
        .collect();

    println!("{:?}", applied_first_fold.len());
}

fn solution_2(inputs: &(CoordinateSet, FoldingInstructions)) {
    let output = inputs.1.iter().fold(inputs.0.clone(), |acc, fold| {
        acc.iter()
            .map(|(mut x, mut y)| {
                match fold.0 {
                    Axis::Horizontal => {
                        if x > fold.1 {
                            x += (fold.1 - x) * 2;
                        }
                    }
                    Axis::Vertical => {
                        if y > fold.1 {
                            y += (fold.1 - y) * 2;
                        }
                    }
                }
                (x, y)
            })
            .collect()
    });

    let highest = output.iter().fold((0, 0), |(mut x, mut y), o| {
        if o.0 > x {
            x = o.0;
        }

        if o.1 > y {
            y = o.1;
        }

        (x, y)
    });

    let mut final_vec = vec![vec![' '; highest.0 as usize + 3]; highest.1 as usize + 3];

    output.iter().for_each(|o| {
        final_vec[o.1 as usize + 1][o.0 as usize + 1] = '█';
    });

    final_vec
        .iter()
        .for_each(|l| println!("{}", l.iter().collect::<String>()));
}

fn read_inputs<T: AsRef<Path>>(path: T) -> (CoordinateSet, FoldingInstructions) {
    let file_content = fs::read_to_string(path).expect("Input file not found.");

    let (points, instructions) = file_content.split_once("\n\n").unwrap();

    let point_set: HashSet<(i32, i32)> = points
        .lines()
        .map(|l| {
            l.split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let instruction_vec = instructions
        .lines()
        .map(|s| {
            let only_expression = s.chars().skip(11).collect::<String>();
            let (axis_str, num_str) = only_expression.split_once('=').unwrap();

            let axis = if axis_str == "x" {
                Axis::Horizontal
            } else {
                Axis::Vertical
            };

            (axis, num_str.parse::<i32>().unwrap())
        })
        .collect();

    (point_set, instruction_vec)
}
