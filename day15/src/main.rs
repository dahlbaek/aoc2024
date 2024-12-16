use core::str;
use std::{collections::HashMap, iter::successors, thread, time::Duration};

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

fn get(map: &[u8], width: isize, (row, col): Position) -> u8 {
    map[as_usize(col + (width + 1) * row)]
}

fn set(map: &mut [u8], width: isize, (row, col): Position, value: u8) {
    map[as_usize(col + (width + 1) * row)] = value
}

fn as_position(u: usize, width: isize) -> Position {
    let i: isize = u.try_into().unwrap();
    (i / (width + 1), i % (width + 1))
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

fn parse2() -> Vec<u8> {
    PUZZLE
        .split_once("\n\n")
        .unwrap()
        .0
        .bytes()
        .flat_map(|b| match b {
            b'#' => vec![b'#', b'#'].into_iter(),
            b'O' => vec![b'[', b']'].into_iter(),
            b'.' => vec![b'.', b'.'].into_iter(),
            b'@' => vec![b'@', b'.'].into_iter(),
            b'\n' => vec![b'\n'].into_iter(),
            _ => panic!("unknown byte: {}", str::from_utf8(&[b]).unwrap()),
        })
        .collect()
}

fn go(position: &Position, direction: &Direction, height: isize, width: isize) -> Option<Position> {
    Some((position.0 + direction.0, position.1 + direction.1))
        .filter(|&(row, col)| row >= 0 && row < height && col >= 0 && col < width)
}

fn run_instruction(mut map: Vec<u8>, dim: isize, direction: &Direction) -> Vec<u8> {
    let robot = as_position(map.iter().position(|&b| b == b'@').unwrap(), dim);
    let iterations = successors(Some(robot), |position| go(position, direction, dim, dim))
        .take_while(|&position| get(&map, dim, position) != b'#')
        .position(|position| get(&map, dim, position) == b'.')
        .map(|n| n + 1);

    if let Some(n) = iterations {
        let mut iter =
            successors(Some(robot), |position| go(position, direction, dim, dim)).take(n);

        let first = iter.next().unwrap();
        set(&mut map, dim, first, b'.');

        let second = iter.next().unwrap();
        set(&mut map, dim, second, b'@');

        iter.for_each(|position| set(&mut map, dim, position, b'O'));
    }

    if DEBUG {
        thread::sleep(Duration::from_micros(10));
        println!("{}\n", str::from_utf8(&map).unwrap(),);
    }

    map
}

fn move_ns<'a>(
    map: &'a [u8],
    height: isize,
    width: isize,
    direction: &'a Direction,
) -> impl Fn(&HashMap<Position, (u8, u8)>) -> Option<HashMap<Position, (u8, u8)>> + 'a {
    move |positions| {
        positions
            .iter()
            .filter(|(_, (old, _))| *old != b'.')
            .try_fold(
                HashMap::new(),
                |mut next_positions, (position, (old, _))| {
                    let next_position = go(position, direction, height, width).unwrap();
                    match get(map, width, next_position) {
                        b'[' => {
                            next_positions.entry(next_position).or_insert((b'[', *old));
                            let e_pos = go(position, &EAST, height, width).unwrap();
                            let e_val = positions.get(&e_pos).map(|b| b.0).unwrap_or(b'.');
                            let next_e_pos = go(&next_position, &EAST, height, width).unwrap();
                            next_positions.entry(next_e_pos).or_insert((b']', e_val));
                            Some(next_positions)
                        }
                        b']' => {
                            next_positions.entry(next_position).or_insert((b']', *old));
                            let w_pos = go(position, &WEST, height, width).unwrap();
                            let w_val = positions.get(&w_pos).map(|b| b.0).unwrap_or(b'.');
                            let next_we_pos = go(&next_position, &WEST, height, width).unwrap();
                            next_positions.entry(next_we_pos).or_insert((b'[', w_val));
                            Some(next_positions)
                        }
                        b'.' => {
                            next_positions.insert(next_position, (b'.', *old));
                            Some(next_positions)
                        }
                        b'#' => None,
                        _ => panic!(),
                    }
                },
            )
            .filter(|next_positions| !next_positions.is_empty())
    }
}

fn run_instruction2(mut map: Vec<u8>, h: isize, w: isize, direction: &Direction) -> Vec<u8> {
    let robot = as_position(map.iter().position(|&b| b == b'@').unwrap(), w);
    if [EAST, WEST].contains(direction) {
        let iterations = successors(Some(robot), |&position| go(&position, direction, h, w))
            .take_while(|&position| get(&map, w, position) != b'#')
            .position(|position| get(&map, w, position) == b'.')
            .map(|n| n + 1);

        if let Some(n) = iterations {
            let mut value = b'.';
            for pos in successors(Some(robot), |pos| go(pos, direction, h, w)).take(n) {
                let old_value = value;
                value = get(&map, w, pos);
                set(&mut map, w, pos, old_value);
            }
        }
    } else {
        let positions = Some(HashMap::from([(robot, (b'@', b'.'))]));
        let mapping = successors(positions, move_ns(&map, h, w, direction)).collect::<Vec<_>>();
        if mapping.last().unwrap().iter().all(|(_, (b, _))| *b == b'.') {
            mapping
                .into_iter()
                .flatten()
                .for_each(|(pos, (_, new))| set(&mut map, w, pos, new));
        }
    }

    if DEBUG {
        thread::sleep(Duration::from_micros(10));
        println!("{}\n", str::from_utf8(&map).unwrap(),);
    }

    map
}

fn main() {
    let (map1, instructions) = parse();
    let part1 = instructions
        .iter()
        .fold(map1, |acc, i| run_instruction(acc, DIM, i))
        .into_iter()
        .enumerate()
        .filter(|(_, value)| *value == b'O')
        .map(|(index, _)| as_position(index, DIM))
        .map(|(row, col)| 100 * row + col)
        .sum::<isize>();
    println!("Part 1: {}", part1);

    let map2 = parse2();
    let part2 = instructions
        .iter()
        .fold(map2, |acc, i| run_instruction2(acc, DIM, 2 * DIM, i))
        .into_iter()
        .enumerate()
        .filter(|(_, value)| *value == b'[')
        .map(|(index, _)| as_position(index, 2 * DIM))
        .map(|(row, col)| 100 * row + col)
        .sum::<isize>();
    println!("Part 2: {}", part2);
}
