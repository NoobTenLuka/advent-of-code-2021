use std::{fs, path::Path, time::Instant};

use cached::proc_macro::cached;
use itertools::Itertools;

pub fn run() {
    let inputs = read_inputs("inputs/21-input.txt");

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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Player {
    pub position: u32,
    pub score: u32,
}

fn solution_1(inputs: &(u32, u32)) {
    let mut player_1 = Player {
        position: inputs.0,
        score: 0,
    };
    let mut player_2 = Player {
        position: inputs.1,
        score: 0,
    };
    let mut number_of_rolls = 0;
    let mut is_player_ones_turn = true;

    loop {
        let roll: u32 = (0..3)
            .map(|_| {
                number_of_rolls += 1;
                number_of_rolls % 100
            })
            .sum();
        let player_ref = if is_player_ones_turn {
            &mut player_1
        } else {
            &mut player_2
        };
        player_ref.position = (player_ref.position + roll - 1) % 10 + 1;

        player_ref.score += player_ref.position;

        if player_ref.score >= 1000 {
            break;
        }

        is_player_ones_turn = !is_player_ones_turn;
    }

    let losing_player_score = if player_1.score >= 1000 {
        player_2.score
    } else {
        player_1.score
    };

    println!("The product is {}", number_of_rolls * losing_player_score);
}

fn solution_2(inputs: &(u32, u32)) {
    let result = (3..=9).fold((0, 0), |acc, i| {
        let output = solution_helper(
            i,
            true,
            Player {
                position: inputs.0,
                score: 0,
            },
            Player {
                position: inputs.1,
                score: 0,
            },
        );
        (acc.0 + output.0, acc.1 + output.1)
    });

    println!(
        "The player with the most wins does so in {} universes",
        result.0.max(result.1)
    );
}

#[cached]
fn solution_helper(
    sum_of_rolls: u32,
    is_player_ones_turn: bool,
    player_1: Player,
    player_2: Player,
) -> (u64, u64) {
    let player_ref = if is_player_ones_turn {
        &player_1
    } else {
        &player_2
    };

    let new_pos = (player_ref.position + sum_of_rolls - 1) % 10 + 1;
    let new_score = player_ref.score + new_pos;

    if new_score >= 21 {
        if is_player_ones_turn {
            return (get_factor(sum_of_rolls), 0);
        } else {
            return (0, get_factor(sum_of_rolls));
        }
    };

    let (new_player_1, new_player_2) = if is_player_ones_turn {
        (
            Player {
                position: new_pos,
                score: new_score,
            },
            player_2,
        )
    } else {
        (
            player_1,
            Player {
                position: new_pos,
                score: new_score,
            },
        )
    };

    (3..=9).fold((0, 0), |acc, i| {
        let output = solution_helper(i, !is_player_ones_turn, new_player_1, new_player_2);
        (
            acc.0 + output.0 * get_factor(sum_of_rolls),
            acc.1 + output.1 * get_factor(sum_of_rolls),
        )
    })
}
fn get_factor(input: u32) -> u64 {
    match input {
        3 => 1,
        4 => 3,
        5 => 6,
        6 => 7,
        7 => 6,
        8 => 3,
        9 => 1,
        _ => 0,
    }
}

fn read_inputs<T: AsRef<Path>>(path: T) -> (u32, u32) {
    let file_content = fs::read_to_string(path).expect("Input file not found.");

    file_content
        .lines()
        .map(|l| l.chars().last().unwrap().to_digit(10).unwrap())
        .collect_tuple()
        .unwrap()
}
