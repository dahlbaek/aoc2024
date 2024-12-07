const PUZZLE: &str = include_str!("puzzle");
const BIT_MASK: [usize; 11] = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024];

fn parse(s: &str) -> (usize, Vec<usize>) {
    let (target, raw_numbers) = s.split_once(": ").unwrap();
    let numbers = raw_numbers
        .split(" ")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();
    (target.parse().unwrap(), numbers)
}

fn could_be_true1((target, numbers): &(usize, Vec<usize>)) -> bool {
    (0..2 << numbers.len()).any(|j| {
        let acc =
            numbers[1..]
                .iter()
                .zip(BIT_MASK.iter())
                .fold(numbers[0], |acc, (number, mask)| {
                    if j & mask == 0 {
                        acc + number
                    } else {
                        acc * number
                    }
                });
        acc == *target
    })
}

fn could_be_true2((target, numbers): (usize, &[usize]), current: usize) -> bool {
    if numbers.is_empty() {
        current == target
    } else {
        could_be_true2((target, &numbers[1..]), current + numbers[0])
            || could_be_true2((target, &numbers[1..]), current * numbers[0])
            || could_be_true2(
                (target, &numbers[1..]),
                (current.to_string() + &numbers[0].to_string())
                    .parse()
                    .unwrap(),
            )
    }
}

fn main() {
    let part1 = PUZZLE
        .trim()
        .lines()
        .map(parse)
        .filter(could_be_true1)
        .map(|(target, _)| target)
        .sum::<usize>();
    println!("Part 1: {}", part1);

    let part2 = PUZZLE
        .trim()
        .lines()
        .map(parse)
        .filter(|(target, numbers)| could_be_true2((*target, &numbers[1..]), numbers[0]))
        .map(|(target, _)| target)
        .sum::<usize>();
    println!("Part 2: {}", part2);
}
