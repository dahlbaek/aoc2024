use std::{collections::HashMap, iter::successors};

const PUZZLE: &str = include_str!("puzzle");

const DIM: isize = 71;
const BYTES: usize = 1024;

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn neighbors(&self) -> impl Iterator<Item = Position> {
        let x = self.x;
        let y = self.y;
        [
            Position { x, y: y + 1 },
            Position { x, y: y - 1 },
            Position { x: x + 1, y },
            Position { x: x - 1, y },
        ]
        .into_iter()
        .filter(|pos| pos.x >= 0 && pos.x < DIM && pos.y >= 0 && pos.y < DIM)
    }

    fn as_index(&self) -> usize {
        into_usize(self.x + self.y * DIM)
    }
}

fn parse() -> Vec<Position> {
    PUZZLE
        .trim()
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            Position {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .take(BYTES)
        .collect()
}

fn into_usize(i: isize) -> usize {
    i.try_into().unwrap()
}

fn new_grid(fallen: &[Position]) -> Vec<bool> {
    let mut grid = vec![true; into_usize(DIM * DIM)];
    fallen.iter().for_each(|p| grid[p.as_index()] = false);
    grid
}

fn next_step(grid: Vec<bool>) -> impl FnMut(&Vec<Position>) -> Option<Vec<Position>> {
    let mut seen = HashMap::new();
    let mut current_steps = 0;
    move |previous| {
        seen.extend(previous.iter().map(|&p| (p, current_steps)));
        current_steps += 1;
        let mut next = Vec::new();
        for pos in previous {
            for neighbor in pos.neighbors() {
                if !seen.contains_key(&neighbor) && grid[neighbor.as_index()] {
                    seen.insert(neighbor, current_steps);
                    next.push(neighbor);
                }
            }
        }
        Some(next).filter(|n| !n.is_empty())
    }
}

fn main() {
    let start = Position { x: 0, y: 0 };
    let end = Position {
        x: DIM - 1,
        y: DIM - 1,
    };
    let parsed = parse();
    let grid = new_grid(&parsed);
    let part1 = successors(Some(vec![start]), next_step(grid))
        .enumerate()
        .find(|(_, positions)| positions.contains(&end))
        .unwrap()
        .0;
    println!("Part 1: {}", part1);
}
