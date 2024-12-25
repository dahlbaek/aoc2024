use core::str;
use std::{collections::HashMap, iter::repeat};

const PUZZLE: &str = include_str!("puzzle");

fn count_part1(i: u64, left: u32) -> u64 {
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

fn next_elements(element: u64) -> Vec<u64> {
    if element == 0 {
        vec![1]
    } else {
        let element_string = element.to_string();
        let element_len = element_string.len();
        if element_len % 2 == 0 {
            vec![
                element_string[..element_len / 2].parse().unwrap(),
                element_string[element_len / 2..].parse().unwrap(),
            ]
        } else {
            vec![element * 2024]
        }
    }
}

fn next_counts(acc: &HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut next_acc = HashMap::new();
    for (&i, &v) in acc {
        for next_element in next_elements(i) {
            *next_acc.entry(next_element).or_default() += v;
        }
    }
    next_acc
}

fn main() {
    let parsed = PUZZLE
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();

    let part1 = parsed.iter().map(|&i| count_part1(i, 25)).sum::<u64>();
    println!("Part 1: {}", part1);

    let initial_counts = parsed.iter().cloned().zip(repeat(1)).collect();
    let part2 = (0..75)
        .fold(initial_counts, |acc, _| next_counts(&acc))
        .values()
        .sum::<u64>();
    println!("Part 2: {}", part2);
}
