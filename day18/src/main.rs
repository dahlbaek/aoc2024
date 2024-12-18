use std::{collections::HashSet, iter::successors};

const PUZZLE: &str = include_str!("puzzle");

const DIM: isize = 71;

const START: Position = Position { x: 0, y: 0 };
const END: Position = Position {
    x: DIM - 1,
    y: DIM - 1,
};

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
        .collect()
}

fn new_blocked(fallen: &[Position], limit: usize) -> HashSet<Position> {
    fallen.iter().take(limit).cloned().collect()
}

fn next_step(blocked: HashSet<Position>) -> impl FnMut(&Vec<Position>) -> Option<Vec<Position>> {
    let mut seen = HashSet::from([START]);
    move |previous| {
        let mut next = Vec::new();
        for pos in previous {
            for neighbor in pos.neighbors() {
                if !seen.contains(&neighbor) && !blocked.contains(&neighbor) {
                    seen.insert(neighbor);
                    next.push(neighbor);
                }
            }
        }
        Some(next).filter(|n| !n.is_empty())
    }
}

fn find_steps(parsed: &[Position], limit: usize) -> Option<usize> {
    let blocked = new_blocked(parsed, limit);

    successors(Some(vec![START]), next_step(blocked))
        .enumerate()
        .find(|(_, positions)| positions.contains(&END))
        .map(|(index, _)| index)
}

fn main() {
    let parsed = parse();

    println!("Part 1: {}", find_steps(&parsed, 1024).unwrap());

    let index = (0..parsed.len())
        .collect::<Vec<_>>()
        .partition_point(|&limit| find_steps(&parsed, limit).is_some());
    let part2 = parsed[index - 1];
    println!("Part 2: {},{}", part2.x, part2.y)
}
