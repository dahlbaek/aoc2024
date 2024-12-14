use core::str;

const PUZZLE: &str = include_str!("puzzle");

const W: isize = 101;
const T: isize = 103;

type Position = (isize, isize);
type Velocity = (isize, isize);

fn parse_tuple(s: &str) -> (isize, isize) {
    let (x, y) = s[2..].split_once(',').unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}

fn parse(s: &str) -> (Position, Velocity) {
    let (p, v) = s.split_once(' ').unwrap();
    (parse_tuple(p), parse_tuple(v))
}

fn normalize(i: isize, l: isize) -> isize {
    let mut rem = i % l;
    if rem < 0 {
        rem += l
    }
    rem
}

fn elapse(seconds: isize) -> impl Fn((Position, Velocity)) -> Position {
    move |(position, velocity)| {
        (
            normalize(position.0 + seconds * velocity.0, W),
            normalize(position.1 + seconds * velocity.1, T),
        )
    }
}

type QuadrantCount = [isize; 4];

fn split_quadrants(mut acc: QuadrantCount, position: Position) -> QuadrantCount {
    if position.0 < W / 2 && position.1 < T / 2 {
        acc[0] += 1
    } else if position.0 > W / 2 && position.1 < T / 2 {
        acc[1] += 1
    } else if position.0 < W / 2 && position.1 > T / 2 {
        acc[2] += 1
    } else if position.0 > W / 2 && position.1 > T / 2 {
        acc[3] += 1
    }
    acc
}

type Screen = [u8; ((W + 1) * T) as usize];

fn new_screen() -> Screen {
    let mut screen = [b'.'; ((W + 1) * T) as usize];
    for t in 0..T {
        screen[(t * (W + 1) + W) as usize] = b'\n'
    }
    screen
}

fn build_screen(mut screen: Screen, position: Position) -> Screen {
    screen[(position.0 + position.1 * (W + 1)) as usize] = b'*';
    screen
}

fn display(screen: Screen) {
    println!("{}", str::from_utf8(&screen).unwrap());
}

fn main() {
    let part1 = PUZZLE
        .trim()
        .lines()
        .map(parse)
        .map(elapse(100))
        .fold(QuadrantCount::default(), split_quadrants)
        .into_iter()
        .product::<isize>();
    println!("Part 1: {}", part1);

    for seconds in 0.. {
        let screen = PUZZLE
            .trim()
            .lines()
            .map(parse)
            .map(elapse(seconds))
            .fold(new_screen(), build_screen);
        // ranges found by displaying screens for low values and finding
        // high concentrations in width and height separately
        let count = (35..=65)
            .flat_map(|w| (30..=62).map(move |t| (w, t)))
            .filter(|(w, t)| screen[(w + t * (W + 1)) as usize] == b'*')
            .count();
        if count >= 300 {
            display(screen);
            println!("Part 2: {}", seconds);
            break;
        }
    }
}
