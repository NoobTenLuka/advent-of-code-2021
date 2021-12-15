use std::{fs, path::Path, time::Instant, vec};

use itertools::Itertools;
use pathfinding::prelude::dijkstra;

pub fn run() {
    let inputs = read_inputs("inputs/15-input.txt");

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn successors(&self, map: &[Vec<i32>]) -> Vec<(Pos, i32)> {
        let &Pos(x, y) = self;
        vec![Pos(x + 1, y), Pos(x, y + 1)]
            .into_iter()
            .map(|p| {
                let weight = *map.get(p.1).unwrap_or(&vec![]).get(p.0).unwrap_or(&10000);
                (p, weight)
            })
            .collect()
    }
}

fn solution_1(inputs: &[Vec<i32>]) {
    let goal = Pos(inputs[0].len() - 1, inputs.len() - 1);
    let result = dijkstra(&Pos(0, 0), |p| p.successors(inputs), |p| *p == goal);
    println!("{:?}", result.unwrap().1);
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Sol2Pos(usize, usize);

impl Sol2Pos {
    fn successors(&self, map: &[Vec<i32>]) -> Vec<(Sol2Pos, i32)> {
        let &Sol2Pos(x, y) = self;
        let mut pos = vec![Sol2Pos(x + 1, y), Sol2Pos(x, y + 1)];

        if let Some(sub_x) = x.checked_sub(1) {
            pos.push(Sol2Pos(sub_x, y));
        }

        if let Some(sub_y) = y.checked_sub(1) {
            pos.push(Sol2Pos(x, sub_y));
        }

        pos.into_iter()
            .map(|p| {
                let mut weight = *map
                    .get(p.1 % map.len())
                    .unwrap_or(&vec![])
                    .get(p.0 % map.len())
                    .unwrap_or(&1000);

                weight += (p.0 / map.len()) as i32;
                weight += (p.1 / map.len()) as i32;

                if weight >= 10 {
                    weight -= 9;
                }

                (p, weight)
            })
            .collect()
    }
}

fn solution_2(inputs: &[Vec<i32>]) {
    let goal = Sol2Pos(inputs[0].len() * 5 - 1, inputs.len() * 5 - 1);
    let result = dijkstra(&Sol2Pos(0, 0), |p| p.successors(inputs), |p| *p == goal);
    println!("{:?}", result.unwrap().1);
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
