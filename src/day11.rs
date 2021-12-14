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
            .map(|r| r.iter().map(|e| e + 1).collect_vec())
            .collect_vec();

        let mut sum = acc.0;

        for i in 0..increased.len() {
            for j in 0..increased[i].len() {
                if increased[i][j] > 9 {
                    sum += flash(&mut increased, i as isize, j as isize);
                }
            }
        }

        let final_idk = increased
            .iter()
            .map(|r| {
                r.iter()
                    .map(|e| if *e >= 10 { 0 } else { *e })
                    .collect_vec()
            })
            .collect_vec();
        (sum, final_idk)
    });

    println!("The answer is {}", folding_output.0);
}

fn solution_2(inputs: &[Vec<i32>]) {
    let mut acc = inputs.to_owned();
    let mut count = 0;

    loop {
        count += 1;
        let mut increased = acc
            .iter()
            .map(|r| r.iter().map(|e| e + 1).collect_vec())
            .collect_vec();

        for i in 0..increased.len() {
            for j in 0..increased[i].len() {
                if increased[i][j] > 9 {
                    flash(&mut increased, i as isize, j as isize);
                }
            }
        }

        acc = increased
            .iter()
            .map(|r| {
                r.iter()
                    .map(|e| if *e >= 10 { 0 } else { *e })
                    .collect_vec()
            })
            .collect_vec();

        if acc.iter().map(|r| r.iter().sum::<i32>()).sum::<i32>() == 0 {
            println!("The answer is {}", count);
            break;
        }
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
