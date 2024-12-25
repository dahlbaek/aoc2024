use core::str;
use std::{
    collections::{HashMap, HashSet},
    iter::repeat,
};

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

fn add_elements(mut space: HashSet<u64>, &element: &u64) -> HashSet<u64> {
    let mut elems = vec![element];
    while !elems.is_empty() {
        for &elem in &elems {
            space.insert(elem);
        }
        elems = elems
            .iter()
            .flat_map(|&elem| next_elements(elem))
            .filter(|elem| !space.contains(elem))
            .collect::<Vec<_>>();
    }
    space
}

fn next_counts(acc: HashMap<u64, u64>, space: &HashSet<u64>) -> HashMap<u64, u64> {
    space
        .iter()
        .map(|&i| {
            let count = next_elements(i)
                .iter()
                .map(|&next_element| acc[&next_element])
                .sum::<u64>();
            (i, count)
        })
        .collect()
}

fn main() {
    let parsed = PUZZLE
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();

    let part1 = parsed.iter().map(|&i| count_part1(i, 25)).sum::<u64>();
    println!("Part 1: {}", part1);

    let space = parsed.iter().fold(HashSet::new(), add_elements);
    let first = space.iter().cloned().zip(repeat(1)).collect();
    let counts = (0..75).fold(first, |acc, _| next_counts(acc, &space));
    let part2 = parsed.iter().map(|&i| counts[&i]).sum::<u64>();
    println!("Part 2: {}", part2);
}
