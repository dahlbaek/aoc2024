use std::{
    cmp::min,
    collections::{HashMap, HashSet},
    iter::from_fn,
};

const PUZZLE: &str = include_str!("puzzle");

type Position = (isize, isize);
type CheapestStrategy = HashMap<Position, u64>;

#[derive(Debug, Clone)]
struct Input {
    a: Position,
    b: Position,
    prize: Position,
}

fn parse_button(s: &str) -> (Position, &str) {
    let (_, s) = s.split_once('+').unwrap();
    let (x, s) = s.split_once(',').unwrap();
    let (_, s) = s.split_once('+').unwrap();
    let (y, s) = s.split_once('\n').unwrap();
    ((x.parse().unwrap(), y.parse().unwrap()), s)
}

fn parse() -> Vec<Input> {
    PUZZLE
        .split("\n\n")
        .map(|s| {
            let (a, s) = parse_button(s);
            let (b, s) = parse_button(s);
            let (_, s) = s.split_once('=').unwrap();
            let (x, s) = s.split_once(',').unwrap();
            let (_, y) = s.split_once('=').unwrap();
            let prize = (x.parse().unwrap(), y.parse().unwrap());
            Input { a, b, prize }
        })
        .collect()
}

fn next_positions((x, y): Position, input: &Input) -> [(Position, u64); 2] {
    [
        ((x + input.a.0, y + input.a.1), 3),
        ((x + input.b.0, y + input.b.1), 1),
    ]
}

fn cheapest(input: Input) -> impl FnMut() -> Option<Option<u64>> {
    let mut state = CheapestStrategy::from([((0, 0), 0)]);
    let mut latest_points = HashSet::from([(0, 0)]);
    move || {
        let mut new_latest_points = HashSet::new();
        for &position in &latest_points {
            let tokens = state[&position];
            for (next_position, additional_tokens) in next_positions(position, &input) {
                new_latest_points.insert(next_position);
                let path_tokens = tokens + additional_tokens;
                state
                    .entry(next_position)
                    .and_modify(|e| *e = min(*e, path_tokens))
                    .or_insert(path_tokens);
            }
        }
        latest_points = new_latest_points;
        Some(state.get(&input.prize).cloned())
    }
}

fn part2_input(input: &Input) -> Input {
    Input {
        prize: (
            input.prize.0 + 10000000000000,
            input.prize.1 + 10000000000000,
        ),
        a: input.a,
        b: input.b,
    }
}

fn direct(input: Input) -> Option<isize> {
    let a = input.a;
    let b = input.b;
    let prize = input.prize;

    let det = a.0 * b.1 - a.1 * b.0;
    let x_numerator = prize.0 * b.1 - prize.1 * b.0;
    let y_numerator = prize.1 * a.0 - prize.0 * a.1;

    if x_numerator % det != 0 || y_numerator % det != 0 {
        None
    } else {
        Some(3 * (x_numerator / det) + y_numerator / det)
    }
}

fn main() {
    let parsed = parse();

    let part1 = parsed
        .iter()
        .filter_map(|input| from_fn(cheapest(input.clone())).nth(200).unwrap())
        .sum::<u64>();
    println!("Part 1: {}", part1);

    let part2 = parsed
        .iter()
        .map(part2_input)
        .filter_map(direct)
        .sum::<isize>();
    println!("Part 2: {}", part2)
}
