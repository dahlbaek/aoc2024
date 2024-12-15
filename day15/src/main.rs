use core::str;
use std::{iter::successors, thread, time::Duration};

const PUZZLE: &str = include_str!("puzzle");
const DIM: isize = 50;
const DEBUG: bool = false;

type Position = (isize, isize);
type Direction = (isize, isize);

const EAST: Direction = (0, 1);
const NORTH: Direction = (-1, 0);
const WEST: Direction = (0, -1);
const SOUTH: Direction = (1, 0);

fn parse_direction(&b: &u8) -> Option<Direction> {
    match b {
        b'>' => Some(EAST),
        b'^' => Some(NORTH),
        b'<' => Some(WEST),
        b'v' => Some(SOUTH),
        b'\n' => None,
        _ => panic!("unknown byte: {}", str::from_utf8(&[b]).unwrap()),
    }
}

fn as_usize(i: isize) -> usize {
    i.try_into().unwrap()
}

fn as_isize(u: usize) -> isize {
    u.try_into().unwrap()
}

fn get(map: &[u8], (row, col): Position) -> u8 {
    map[as_usize(col + (DIM + 1) * row)]
}

fn set(map: &mut [u8], (row, col): Position, value: u8) {
    map[as_usize(col + (DIM + 1) * row)] = value
}

fn as_position(u: usize) -> Position {
    (as_isize(u) / (DIM + 1), as_isize(u) % (DIM + 1))
}

fn parse() -> (Vec<u8>, Vec<Direction>) {
    let (raw_map, raw_directions) = PUZZLE.split_once("\n\n").unwrap();
    let map = raw_map.as_bytes().to_vec();
    let directions = raw_directions
        .as_bytes()
        .iter()
        .filter_map(parse_direction)
        .collect();
    (map, directions)
}

fn go(position: Position, direction: &Direction) -> Option<Position> {
    let next = (position.0 + direction.0, position.1 + direction.1);
    if next.0 >= 0 && next.0 < DIM && next.1 >= 0 && next.1 < DIM {
        Some(next)
    } else {
        None
    }
}

fn run_instruction(mut map: Vec<u8>, direction: &Direction) -> Vec<u8> {
    let robot = as_position(map.iter().position(|&b| b == b'@').unwrap());
    let iterations = successors(Some(robot), |&position| go(position, direction))
        .take_while(|&position| get(&map, position) != b'#')
        .position(|position| get(&map, position) == b'.')
        .map(|n| n + 1);

    if let Some(n) = iterations {
        let mut iter = successors(Some(robot), |&position| go(position, direction)).take(n);

        let first = iter.next().unwrap();
        set(&mut map, first, b'.');

        let second = iter.next().unwrap();
        set(&mut map, second, b'@');

        for position in iter {
            set(&mut map, position, b'O');
        }
    }

    if DEBUG {
        thread::sleep(Duration::from_micros(10));
        println!("{}\n", str::from_utf8(&map).unwrap(),);
    }

    map
}

fn main() {
    let (map, instructions) = parse();

    let part1_map = instructions.iter().fold(map, run_instruction);
    let part1 = (0..DIM)
        .flat_map(|x| (0..DIM).map(move |y| (x, y)))
        .filter(|&position| get(&part1_map, position) == b'O')
        .map(|(row, col)| 100 * row + col)
        .sum::<isize>();

    println!("Part 1: {}", part1)
}
