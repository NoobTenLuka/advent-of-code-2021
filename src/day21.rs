use std::{fs, path::Path, time::Instant};

use itertools::Itertools;

pub fn run() {
    let inputs = read_inputs("inputs/21-input.txt");

    let start = Instant::now();
    solution_1(&inputs);
    println!(
        "It took {:?} for the first solution to complete.",
        start.elapsed()
    );
}

#[derive(Debug, Clone, Copy)]
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

fn solution_2(inputs: &(u32, u32)) {}

fn solution_helper(
    outcomes: &[u32],
    mut is_player_ones_turn: bool,
    mut player_1: Player,
    mut player_2: Player,
) -> (u64, u64) {
    let vec: Vec<u32> = Vec::new();
    let mut outcome_wrapper: Vec<u32> = Vec::from(outcomes);
    let player_ref = if is_player_ones_turn {
        &mut player_1
    } else {
        &mut player_2
    };

    if outcomes.len() == 3 {
        player_ref.score += outcomes.iter().fold(0, |acc, x| acc + x);
        is_player_ones_turn = !is_player_ones_turn;
        outcome_wrapper = vec;
    }

    if player_ref.score >= 21 {
        if is_player_ones_turn {
            return (1, 0);
        } else {
            return (0, 1);
        }
    }

    solution_helper(&outcome_wrapper, is_player_ones_turn, player_1, player_2);
    (0, 0)
}

fn read_inputs<T: AsRef<Path>>(path: T) -> (u32, u32) {
    let file_content = fs::read_to_string(path).expect("Input file not found.");

    file_content
        .lines()
        .map(|l| l.chars().last().unwrap().to_digit(10).unwrap())
        .collect_tuple()
        .unwrap()
}
