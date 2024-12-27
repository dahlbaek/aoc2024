use std::{cmp::max, collections::HashMap, iter::once};

const PUZZLE: &str = include_str!("puzzle");

const ANUM: Position = Position { x: 2, y: 3 };
const UNDEFINEDNUM: Position = Position { x: 0, y: 3 };

const UP: Position = Position { x: 1, y: 0 };
const A: Position = Position { x: 2, y: 0 };
const LEFT: Position = Position { x: 0, y: 1 };
const DOWN: Position = Position { x: 1, y: 1 };
const RIGHT: Position = Position { x: 2, y: 1 };
const BUTTONS: [Position; 5] = [UP, A, LEFT, DOWN, RIGHT];
const UNDEFINED: Position = Position { x: 0, y: 0 };

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct Position {
    x: isize,
    y: isize,
}

fn show_path(buttons: &Vec<Position>) -> String {
    let mut s = String::new();
    for button in buttons {
        let b = show_dir_button(*button);
        s.push(b);
    }
    s
}

fn show_dir_button(button: Position) -> char {
    match button {
        crate::UP => '^',
        crate::A => 'A',
        crate::LEFT => '<',
        crate::DOWN => 'v',
        crate::RIGHT => '>',
        _ => panic!(),
    }
}

fn show_num_button(button: Position) -> char {
    match button {
        Position { x: 0, y: 0 } => '7',
        Position { x: 1, y: 0 } => '8',
        Position { x: 2, y: 0 } => '9',
        Position { x: 0, y: 1 } => '4',
        Position { x: 1, y: 1 } => '5',
        Position { x: 2, y: 1 } => '6',
        Position { x: 0, y: 2 } => '1',
        Position { x: 1, y: 2 } => '2',
        Position { x: 2, y: 2 } => '3',
        Position { x: 1, y: 3 } => '0',
        Position { x: 2, y: 3 } => 'A',
        _ => panic!(),
    }
}

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
    // .inspect(|c| println!("code: {:?}", c))
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
        if pos.x > button2.x {
            let next_pos = Position {
                x: pos.x - 1,
                y: pos.y,
            };
            if next_pos != forbidden {
                let mut next_current = partial_path.clone();
                next_current.push(LEFT);
                stack.push((next_current, next_pos));
            }
        }
        if pos.x < button2.x {
            let next_pos = Position {
                x: pos.x + 1,
                y: pos.y,
            };
            if next_pos != forbidden {
                let mut next_current = partial_path.clone();
                next_current.push(RIGHT);
                stack.push((next_current, next_pos));
            }
        }
        if pos.y > button2.y {
            let next_pos = Position {
                x: pos.x,
                y: pos.y - 1,
            };
            if next_pos != forbidden {
                let mut next_current = partial_path.clone();
                next_current.push(UP);
                stack.push((next_current, next_pos));
            }
        }
        if pos.y < button2.y {
            let next_pos = Position {
                x: pos.x,
                y: pos.y + 1,
            };
            if next_pos != forbidden {
                let mut next_current = partial_path.clone();
                next_current.push(DOWN);
                stack.push((next_current, next_pos));
            }
        }
        if pos == button2 {
            let mut path = partial_path.clone();
            path.push(A);
            paths.push(path);
        }
    }
    paths.into_iter()
}

fn all_paths_numeric(button1: Position, button2: Position) -> impl Iterator<Item = Vec<Position>> {
    all_paths(button1, button2, UNDEFINEDNUM)
    // .inspect(move |p| {
    //     println!(
    //         "path from {} to {}: {}",
    //         show_num_button(button1),
    //         show_num_button(button2),
    //         show_path(p)
    //     )
    // })
}

fn all_paths_directional(
    button1: Position,
    button2: Position,
) -> impl Iterator<Item = Vec<Position>> {
    all_paths(button1, button2, UNDEFINED)
    // .inspect(|p| println!("path: {:?}", p))
}

fn next_shortest_paths(shortest_paths: &ShortestPaths) -> ShortestPaths {
    // println!("next_shortest_paths");
    pairs()
        .map(|(button1, button2)| {
            let shortest_path = all_paths_directional(button1, button2)
                .map(|p| {
                    p.windows(2)
                        .map(|w| shortest_paths[&(w[0], w[1])])
                        .sum::<usize>()
                })
                .min()
                .unwrap();
            ((button1, button2), shortest_path)
        })
        // .inspect(|i| println!("inside: {:?}", i))
        .collect()
}

fn complexity(code: Vec<Position>, number: usize, shortest_paths: &ShortestPaths) -> usize {
    let shortest = code
        .windows(2)
        .map(|w| {
            all_paths_numeric(w[0], w[1])
                .map(|p| {
                    p.windows(2)
                        .map(|w| shortest_paths[&(w[0], w[1])])
                        .sum::<usize>()
                })
                .min()
                .unwrap()
        })
        .sum::<usize>();
    // println!("shortest*number: {}*{}", shortest, number);
    shortest * number
}

fn part1() -> usize {
    let first = HashMap::from_iter(pairs().map(|p| (p, 1)));
    // println!("first");
    // for i in &first {
    //     println!("inside: {:?}", i)
    // }
    let shortest_paths = (0..2).fold(first, |acc, _| next_shortest_paths(&acc));
    parse()
        .map(|(code, number)| complexity(code, number, &shortest_paths))
        .sum()
}

fn part2() -> usize {
    let first = HashMap::from_iter(pairs().map(|p| (p, 1)));
    // println!("first");
    // for i in &first {
    //     println!("inside: {:?}", i)
    // }
    let shortest_paths = (0..25).fold(first, |acc, _| next_shortest_paths(&acc));
    parse()
        .map(|(code, number)| complexity(code, number, &shortest_paths))
        .sum()
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
