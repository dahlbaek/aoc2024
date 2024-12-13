use std::cmp::min;

const PUZZLE: &str = include_str!("puzzle");

type Position = (isize, isize);

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

fn cheapest(input: &Input) -> Option<isize> {
    let mut min_price = None;
    for a in 0..=100 {
        for b in 0..=100 {
            if input.prize == (a * input.a.0 + b * input.b.0, a * input.a.1 + b * input.b.1) {
                min_price = min_price.map(|c| min(c, a * 3 + b)).or(Some(a * 3 + b));
            }
        }
    }
    min_price
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

    let part1 = parsed.iter().filter_map(cheapest).sum::<isize>();
    println!("Part 1: {}", part1);

    let part2 = parsed
        .iter()
        .map(part2_input)
        .filter_map(direct)
        .sum::<isize>();
    println!("Part 2: {}", part2)
}
