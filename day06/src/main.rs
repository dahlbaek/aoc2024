use std::collections::HashSet;

const PUZZLE: &[u8] = include_bytes!("puzzle");
const DIM: usize = 130;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn next(&self, (col, row): (usize, usize)) -> Option<(usize, usize)> {
        let r = match self {
            Direction::North => row.checked_sub(1).map(|r| (col, r)),
            Direction::South => Some((col, row + 1)),
            Direction::East => Some((col + 1, row)),
            Direction::West => col.checked_sub(1).map(|c| (c, row)),
        };
        r.filter(|&(c, r)| c < DIM && r < DIM)
    }

    fn rotate(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

fn get((col, row): (usize, usize)) -> u8 {
    PUZZLE[col + (DIM + 1) * row]
}

fn get_start() -> (usize, usize) {
    let start_raw = PUZZLE.iter().position(|&b| b == b'^').unwrap();
    (start_raw % (DIM + 1), start_raw / (DIM + 1))
}

fn part1() -> usize {
    let mut current = get_start();
    let mut direction = Direction::North;
    let mut acc = HashSet::<(usize, usize)>::new();
    acc.insert(current);
    while let Some(next) = direction.next(current) {
        if get(next) == b'#' {
            direction = direction.rotate();
        } else {
            current = next;
            acc.insert(current);
        }
    }
    acc.len()
}

fn part2(obstacle: (usize, usize)) -> bool {
    let mut current = get_start();
    let mut direction = Direction::North;
    let mut acc = HashSet::<((usize, usize), Direction)>::new();
    acc.insert((current, direction));
    while let Some(next) = direction.next(current) {
        if acc.contains(&(next, direction)) {
            return true;
        };
        acc.insert((next, direction));
        if next == obstacle || get(next) == b'#' {
            direction = direction.rotate();
        } else {
            current = next;
        }
    }
    false
}

fn main() {
    println!("Part 1: {}", part1());

    // oops, remember to --release
    let mut agg = 0;
    for i in 0..DIM {
        for j in 0..DIM {
            if get((i, j)) != b'^' && part2((i, j)) {
                agg += 1;
            }
        }
    }
    println!("Part 2: {}", agg);
}
