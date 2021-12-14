use std::{fs, path::Path, time::Instant};

use itertools::Itertools;

pub fn run() {
    let inputs = read_inputs("inputs/11-input.txt");

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

fn solution_1(inputs: &[Vec<i32>]) {
    let folding_output = (0..100).fold((0, inputs.to_owned()), |acc, _| {
        let mut increased = acc
            .1
            .iter()
            .map(|r| {
                r.iter()
                    .map(|mut e| {
                        if *e >= 10 {
                            e = &0;
                        }
                        e + 1
                    })
                    .collect_vec()
            })
            .collect_vec();

        let mut sum = acc.0;

        for i in 0..increased.len() {
            for j in 0..increased[i].len() {
                if increased[i][j] > 9 {
                    sum += flash(&mut increased, i as isize, j as isize);
                }
            }
        }

        (sum, increased)
    });

    println!("The answer is {}", folding_output.0);
}

fn solution_2(inputs: &[Vec<i32>]) {
    let mut acc = inputs.to_owned();
    let mut count = 0;

    loop {
        let mut p_count = 0;
        acc = acc
            .iter()
            .map(|r| {
                r.iter()
                    .map(|mut e| {
                        if *e >= 10 {
                            e = &0;
                        }
                        p_count += e;
                        e + 1
                    })
                    .collect_vec()
            })
            .collect_vec();

        if p_count == 0 {
            println!("The answer is {}", count);
            break;
        }

        for i in 0..acc.len() {
            for j in 0..acc[i].len() {
                if acc[i][j] > 9 {
                    flash(&mut acc, i as isize, j as isize);
                }
            }
        }

        count += 1;
    }
}

fn flash(vec: &mut [Vec<i32>], row: isize, column: isize) -> usize {
    if let Some(r) = vec.get_mut(row as usize) {
        if let Some(c) = r.get_mut(column as usize) {
            *c += 1;
            if *c == 10 || *c == 11 {
                *c += 5;
                return 1
                    + flash(vec, row + 1, column)
                    + flash(vec, row - 1, column)
                    + flash(vec, row, column + 1)
                    + flash(vec, row, column - 1)
                    + flash(vec, row + 1, column + 1)
                    + flash(vec, row + 1, column - 1)
                    + flash(vec, row - 1, column + 1)
                    + flash(vec, row - 1, column - 1);
            }
        }
    }
    0
}

fn read_inputs<T: AsRef<Path>>(path: T) -> Vec<Vec<i32>> {
    let file_content = fs::read_to_string(path).expect("Input file not found.");

    file_content
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect_vec()
        })
        .collect_vec()
}
