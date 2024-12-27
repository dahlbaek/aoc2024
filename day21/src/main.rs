use std::{collections::HashMap, iter::once};

use aoc::{tc_dist, Direction, Grid, Position};

const PUZZLE: &str = include_str!("puzzle");

const ANUM: Position = Position { x: 2, y: 3 };
const FORBIDDENNUM: Position = Position { x: 0, y: 3 };

const UP: Position = Position { x: 1, y: 0 };
const A: Position = Position { x: 2, y: 0 };
const LEFT: Position = Position { x: 0, y: 1 };
const DOWN: Position = Position { x: 1, y: 1 };
const RIGHT: Position = Position { x: 2, y: 1 };
const BUTTONS: [Position; 5] = [UP, A, LEFT, DOWN, RIGHT];
const FORBIDDEN: Position = Position { x: 0, y: 0 };

const GRID: Grid = Grid::new(3, 4);

fn get_pos(b: u8) -> Position {
    match b {
        b'7' => Position { x: 0, y: 0 },
        b'8' => Position { x: 1, y: 0 },
        b'9' => Position { x: 2, y: 0 },
        b'4' => Position { x: 0, y: 1 },
        b'5' => Position { x: 1, y: 1 },
        b'6' => Position { x: 2, y: 1 },
        b'1' => Position { x: 0, y: 2 },
        b'2' => Position { x: 1, y: 2 },
        b'3' => Position { x: 2, y: 2 },
        b'0' => Position { x: 1, y: 3 },
        b'A' => Position { x: 2, y: 3 },
        _ => panic!(),
    }
}

fn pairs() -> impl Iterator<Item = (Position, Position)> {
    BUTTONS
        .iter()
        .flat_map(|&b| BUTTONS.iter().map(move |&b2| (b, b2)))
}

fn parse() -> impl Iterator<Item = (Vec<Position>, usize)> {
    PUZZLE.trim().lines().map(|s| {
        let buttons = once(ANUM).chain(s.bytes().map(get_pos)).collect();
        let number = s[..3].parse().unwrap();
        (buttons, number)
    })
}

type ShortestPaths = HashMap<(Position, Position), usize>;

fn all_paths(
    button1: Position,
    button2: Position,
    forbidden: Position,
) -> impl Iterator<Item = Vec<Position>> {
    let mut stack = vec![(vec![A], button1)];
    let mut paths = Vec::new();
    while let Some((partial_path, pos)) = stack.pop() {
        if pos == button2 {
            let mut path = partial_path.clone();
            path.push(A);
            paths.push(path);
        } else {
            for (dir, next_pos) in GRID.neighbours(pos) {
                if tc_dist(next_pos, button2) < tc_dist(pos, button2) && next_pos != forbidden {
                    let mut next_current = partial_path.clone();
                    match dir {
                        Direction::East => next_current.push(RIGHT),
                        Direction::North => next_current.push(UP),
                        Direction::West => next_current.push(LEFT),
                        Direction::South => next_current.push(DOWN),
                    }
                    stack.push((next_current, next_pos));
                }
            }
        }
    }
    paths.into_iter()
}

fn shortest_path_pairs(
    button1: Position,
    button2: Position,
    shortest_paths: &ShortestPaths,
    forbidden: Position,
) -> usize {
    all_paths(button1, button2, forbidden)
        .map(|p| p.windows(2).map(|w| shortest_paths[&(w[0], w[1])]).sum())
        .min()
        .unwrap()
}

fn next_shortest_paths(shortest_paths: &ShortestPaths) -> ShortestPaths {
    pairs()
        .map(|(button1, button2)| {
            let shortest_path = shortest_path_pairs(button1, button2, shortest_paths, FORBIDDEN);
            ((button1, button2), shortest_path)
        })
        .collect()
}

fn shortest_path(code: Vec<Position>, shortest_paths: &ShortestPaths) -> usize {
    code.windows(2)
        .map(|w| shortest_path_pairs(w[0], w[1], shortest_paths, FORBIDDENNUM))
        .sum()
}

fn complexity_sums(keypads: usize) -> usize {
    // Compile all the directional keypads into a single lookup for the shortest path between
    // key pairs on the first directional keypad. Computed from the bottom up, dp style.
    let first = HashMap::from_iter(pairs().map(|p| (p, 1)));
    let shortest_paths = (0..keypads).fold(first, |acc, _| next_shortest_paths(&acc));

    parse()
        .map(|(code, number)| number * shortest_path(code, &shortest_paths))
        .sum()
}

fn main() {
    println!("Part 1: {}", complexity_sums(2));
    println!("Part 2: {}", complexity_sums(25));
}
