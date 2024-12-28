use std::{collections::HashMap, iter::successors};

const PUZZLE: &str = include_str!("puzzle");

fn mix(secret: u64, value: u64) -> u64 {
    secret ^ value
}

fn prune(secret: u64) -> u64 {
    secret % 16777216
}

fn next_secret(mut secret: u64) -> u64 {
    secret = prune(mix(secret, 64 * secret));
    secret = prune(mix(secret, secret / 32));
    secret = prune(mix(secret, 2048 * secret));
    secret
}

fn secret_2000nth(secret: u64) -> u64 {
    (0..2000).fold(secret, |secret, _| next_secret(secret))
}

fn price_changes(secret: u64, changes: usize) -> Vec<(u64, i8)> {
    successors(Some(secret), |&secret| Some(next_secret(secret)))
        .map(|s| i8::try_from(s.to_string().as_bytes().last().unwrap() - 48).unwrap())
        .scan(0, |st, price| {
            let res = price - *st;
            *st = price;
            Some((price.try_into().unwrap(), res))
        })
        .skip(1)
        .take(changes)
        .collect()
}

fn part2(parsed: &[u64]) -> u64 {
    parsed
        .iter()
        .flat_map(|&secret| {
            price_changes(secret, 2000)
                .windows(4)
                .map(|w| ([w[0].1, w[1].1, w[2].1, w[3].1], w[3].0))
                .fold(HashMap::new(), |mut acc, (k, v)| {
                    if !acc.contains_key(&k) {
                        acc.insert(k, v);
                    }
                    acc
                })
        })
        .fold(HashMap::new(), |mut acc, (k, v)| {
            *acc.entry(k).or_default() += v;
            acc
        })
        .values()
        .max()
        .cloned()
        .unwrap()
}

fn main() {
    let parsed = PUZZLE
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<_>>();

    let part1 = parsed.iter().cloned().map(secret_2000nth).sum::<u64>();
    println!("Part 1: {}", part1);

    println!("Part 2: {}", part2(&parsed));
}

#[cfg(test)]
mod tests {
    use crate::{mix, part2, price_changes, prune, secret_2000nth};

    #[test]
    fn test_prune() {
        assert_eq!(prune(100000000), 16113920)
    }

    #[test]
    fn test_mix() {
        assert_eq!(mix(42, 15), 37)
    }

    #[test]
    fn test_secret_2000nth() {
        for (secret, expected) in [
            (1, 8685429),
            (10, 4700978),
            (100, 15273692),
            (2024, 8667524),
        ] {
            assert_eq!(secret_2000nth(secret), expected)
        }
    }

    #[test]
    fn test_price_changes() {
        assert_eq!(
            price_changes(123, 9),
            vec![
                (0, -3),
                (6, 6),
                (5, -1),
                (4, -1),
                (4, 0),
                (6, 2),
                (4, -2),
                (4, 0),
                (2, -2),
            ]
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&vec![1, 2, 3, 2024]), 23)
    }
}
