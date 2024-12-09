const PUZZLE: &str = include_str!("puzzle");

fn is_safe_part1(l: &&str) -> bool {
    l.split_ascii_whitespace()
        .map(|i| i.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
        .windows(2)
        .map(|w| {
            let diff = w[1] - w[0];
            if diff.abs() <= 3 {
                diff
            } else {
                0
            }
        })
        .reduce(|l, r| {
            if l.signum() == r.signum() {
                l.signum() * l.abs().max(r.abs())
            } else {
                0
            }
        })
        .unwrap()
        != 0
}

fn pair_is_unsafe(diff: i64, signum: i64) -> bool {
    diff == 0 || diff.abs() > 3 || diff.signum() != signum
}

fn part2_impl(first: i64, second: i64, tail: &[i64], signum: i64, has_skipped: bool) -> bool {
    if tail.is_empty() {
        return !(has_skipped && pair_is_unsafe(second - first, signum));
    }

    if pair_is_unsafe(second - first, signum) || pair_is_unsafe(tail[0] - second, signum) {
        if has_skipped {
            false
        } else {
            part2_impl(first, tail[0], &tail[1..], signum, true)
                || part2_impl(first, second, &tail[1..], signum, true)
        }
    } else {
        part2_impl(second, tail[0], &tail[1..], signum, has_skipped)
    }
}

fn is_safe_part2(l: &&str) -> bool {
    let ints = l
        .split_ascii_whitespace()
        .map(|i| i.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let peek = ints
        .windows(2)
        .take(3)
        .map(|w| w[1] - w[0])
        .collect::<Vec<i64>>();

    let signum = if peek.iter().filter(|&&i| i > 0).count() > 1 {
        1
    } else if peek.iter().filter(|&&i| i < 0).count() > 1 {
        -1
    } else {
        return false;
    };

    part2_impl(ints[1], ints[2], &ints[3..], signum, true)
        || part2_impl(ints[0], ints[1], &ints[2..], signum, false)
}

fn main() {
    let part1 = PUZZLE.trim().lines().filter(is_safe_part1).count();
    println!("Part 1: {}", part1);

    let part2 = PUZZLE.trim().lines().filter(is_safe_part2).count();
    println!("Part 2: {}", part2);
}
