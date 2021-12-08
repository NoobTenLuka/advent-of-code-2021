use std::{fs, path::Path, time::Instant};

pub fn run() {
    let inputs = read_inputs("inputs/02-input.txt");

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

fn solution_1(inputs: &[(String, i32)]) {
    let (hor, dep) = inputs
        .iter()
        .fold((0i32, 0i32), |(hor, dep), i| match i.0.as_str() {
            "forward" => (hor + i.1, dep),
            "up" => (hor, dep - i.1),
            "down" => (hor, dep + i.1),
            _ => (hor, dep),
        });

    println!(
        "The product of horizontal position and depth is {}.",
        hor * dep
    )
}

fn solution_2(inputs: &[(String, i32)]) {
    let (hor, dep, _) = inputs
        .iter()
        .fold((0i32, 0i32, 0i32), |(hor, dep, aim), i| {
            match i.0.as_str() {
                "forward" => (hor + i.1, dep + (aim * i.1), aim),
                "up" => (hor, dep, aim - i.1),
                "down" => (hor, dep, aim + i.1),
                _ => (hor, dep, aim),
            }
        });

    println!(
        "The product of horizontal position and depth, under consideration of aim, is {}.",
        hor * dep
    )
}

fn read_inputs<T: AsRef<Path>>(path: T) -> Vec<(String, i32)> {
    let file_content = fs::read_to_string(path).expect("Input file not found.");

    file_content
        .split('\n')
        .map(|s| {
            let i = s.split_once(' ').unwrap();

            (i.0.to_string(), i.1.parse::<i32>().unwrap())
        })
        .collect()
}
