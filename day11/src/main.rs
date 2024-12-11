use core::str;

const PUZZLE: &str = include_str!("puzzle");

fn count_part1(i: u64, left: u8) -> u64 {
    if left == 0 {
        1
    } else if i == 0 {
        count_part1(1, left - 1)
    } else {
        let i_string = i.to_string();
        let i_len = i_string.len();
        if i_len % 2 == 0 {
            count_part1(i_string[..i_len / 2].parse().unwrap(), left - 1)
                + count_part1(i_string[i_len / 2..].parse().unwrap(), left - 1)
        } else {
            count_part1(i * 2024, left - 1)
        }
    }
}

fn main() {
    let parsed = PUZZLE
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();

    let part1 = parsed.iter().map(|&i| count_part1(i, 25)).sum::<u64>();
    println!("Part 1: {}", part1);

    let part2 = parsed.iter().map(|&i| count_part1(i, 75)).sum::<u64>();
    println!("Part 2: {}", part2);
}
