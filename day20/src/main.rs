use std::collections::{HashMap, HashSet};

use aoc::{Grid, Position};

const PUZZLE: &[u8] = include_bytes!("puzzle");
const DIM: i64 = 141;
const GRID: Grid = Grid::new(DIM, DIM);
const SAVE_AT_LEAST: u64 = 100;

fn into_usize(i: i64) -> usize {
    i.try_into().unwrap()
}

fn get(position: Position) -> u8 {
    PUZZLE[into_usize(position.x + position.y * (DIM + 1))]
}

fn grid() -> impl Iterator<Item = Position> {
    (0..DIM).flat_map(|x| (0..DIM).map(move |y| Position { x, y }))
}

fn steps_from(start: Position) -> HashMap<Position, u64> {
    let mut current_positions = Vec::new();
    let mut visited = HashSet::new();
    current_positions.push(start);
    visited.insert(start);

    let mut current_steps = 0u64;
    let mut steps = HashMap::new();
    while !current_positions.is_empty() {
        let mut next_current_positions = Vec::new();
        for current_position in current_positions {
            for (_, neighbor) in GRID.neighbours(current_position) {
                if get(neighbor) != b'#' {
                    if !visited.contains(&neighbor) {
                        next_current_positions.push(neighbor);
                    }
                    visited.insert(neighbor);
                }
            }
            steps.insert(current_position, current_steps);
        }
        current_positions = next_current_positions;
        current_steps += 1;
    }
    steps
}

fn part1(start: Position, end: Position) -> usize {
    let steps_from_start = steps_from(start);
    let steps_from_end = steps_from(end);
    let part1_no_cheat = steps_from_start[&end];
    grid()
        .filter(|&pos| get(pos) == b'#')
        .flat_map(|wall| {
            GRID.neighbours(wall)
                .flat_map(move |(_, start2)| {
                    GRID.neighbours(wall)
                        .map(move |(_, end1)| (end1, wall, start2))
                })
                .filter(|(end1, _, start2)| get(*start2) != b'#' && get(*end1) != b'#')
        })
        .map(|(end1, _, start2)| 2 + steps_from_start[&end1] + steps_from_end[&start2])
        .filter(|&total_steps| total_steps + SAVE_AT_LEAST <= part1_no_cheat)
        .count()
}

fn part2(start: Position, end: Position) -> usize {
    let steps_from_start = steps_from(start);
    let steps_from_end = steps_from(end);
    let part1_no_cheat = steps_from_start[&end];
    grid()
        .filter(|&end1| get(end1) != b'#')
        .flat_map(|end1| {
            grid()
                .filter(|&start2| get(start2) != b'#')
                .map(move |start2| {
                    let skipped = (end1.x - start2.x).abs() + (end1.y - start2.y).abs();
                    (start2, u64::try_from(skipped).unwrap())
                })
                .filter(|(_, skipped)| *skipped <= 20)
                .map(move |(start2, skipped)| (end1, start2, skipped))
        })
        .map(|(end1, start2, skipped)| skipped + steps_from_start[&end1] + steps_from_end[&start2])
        .filter(|&total_steps| total_steps + SAVE_AT_LEAST <= part1_no_cheat)
        .count()
}

fn main() {
    let start = grid().find(|&p| get(p) == b'S').unwrap();
    let end = grid().find(|&p| get(p) == b'E').unwrap();

    println!("Part 1: {}", part1(start, end));
    println!("Part 2: {}", part2(start, end));
}
