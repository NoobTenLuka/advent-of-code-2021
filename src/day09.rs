use std::{fs, path::Path, time::Instant};

pub fn run() {
    let inputs = read_inputs("inputs/Input.txt");

    let start = Instant::now();
    solution_1(&inputs);
    println!(
        "It took {:?} for the first solution to complete.",
        start.elapsed()
    );
}

fn solution_1(inputs: &[Vec<i8>]) {
    let sum = inputs.iter().enumerate().fold(0, |acc, (l, line)| {
        acc + line.iter().enumerate().fold(0i32, |acc, (n, num)| {
            if let Some(row_below) = &inputs.get(l + 1) {
                if let Some(el_below) = row_below.get(n) {
                    if num >= el_below {
                        return acc;
                    }
                }
            }

            if let Some(non_overflow) = l.checked_sub(1) {
                if let Some(row_above) = &inputs.get(non_overflow) {
                    if let Some(el_above) = row_above.get(n) {
                        if num >= el_above {
                            return acc;
                        }
                    }
                }
            }

            if let Some(el_right) = line.get(n + 1) {
                if num >= el_right {
                    return acc;
                }
            }

            if let Some(non_overflow) = n.checked_sub(1) {
                if let Some(el_left) = line.get(non_overflow) {
                    if num >= el_left {
                        return acc;
                    }
                }
            }

            acc + (1 + *num as i32)
        })
    });

    println!("The sum of all risks is {}.", &sum);
}

fn read_inputs<T: AsRef<Path>>(path: T) -> Vec<Vec<i8>> {
    let file_content = fs::read_to_string(path).expect("Input file not found.");

    file_content
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect()
        })
        .collect()
}
