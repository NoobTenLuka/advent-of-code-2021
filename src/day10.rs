use std::{collections::VecDeque, fs, path::Path, time::Instant};

pub fn run() {
    let inputs = read_inputs("inputs/10-input.txt");

    let start = Instant::now();
    solution_1(&inputs);
    println!(
        "It took {:?} for the first solution to complete.",
        start.elapsed()
    );
}

#[derive(PartialEq, Clone, Copy)]
enum Open {
    Round = 3,
    Bracket = 57,
    Curly = 1197,
    LT = 25137,
}

fn solution_1(inputs: &[String]) {
    let sum: isize = inputs
        .iter()
        .map(|s| {
            let mut open: VecDeque<Open> = VecDeque::new();
            for c in s.chars() {
                match c {
                    '(' => open.push_front(Open::Round),
                    '[' => open.push_front(Open::Bracket),
                    '{' => open.push_front(Open::Curly),
                    '<' => open.push_front(Open::LT),
                    ')' => {
                        let current_open = open.pop_front().unwrap();
                        if current_open != Open::Round {
                            return 3;
                        }
                    }
                    ']' => {
                        let current_open = open.pop_front().unwrap();
                        if current_open != Open::Bracket {
                            return 57;
                        }
                    }
                    '}' => {
                        let current_open = open.pop_front().unwrap();
                        if current_open != Open::Curly {
                            return 1197;
                        }
                    }
                    '>' => {
                        let current_open = open.pop_front().unwrap();
                        if current_open != Open::LT {
                            return 25137;
                        }
                    }
                    _ => (),
                }
            }
            0
        })
        .sum();

    println!("{}", sum);
}

fn read_inputs<T: AsRef<Path>>(path: T) -> Vec<String> {
    let file_content = fs::read_to_string(path).expect("Input file not found.");

    file_content.split('\n').map(|s| s.to_string()).collect()
}
