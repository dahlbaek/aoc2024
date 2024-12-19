use std::collections::{HashMap, HashSet};

const PUZZLE: &str = include_str!("puzzle");

fn parse() -> (Vec<&'static str>, Vec<&'static str>) {
    let mut lines = PUZZLE.trim().lines();
    let available = lines.next().unwrap().split(", ").collect();
    assert_eq!(lines.next().unwrap(), "");
    let targets = lines.collect();
    (available, targets)
}

fn is_buildable(available: &HashMap<u8, Vec<&str>>, target: &str) -> bool {
    // println!("target: {}", target);
    target.is_empty()
        || available.get(&target.as_bytes()[0]).is_some_and(|v| {
            v.iter().any(|prefix| {
                // println!("prefix: {}", prefix);
                target.starts_with(prefix) && is_buildable(available, &target[prefix.len()..])
            })
        })
}

fn count(available: &HashMap<u8, Vec<&str>>, target: &str) -> u64 {
    // println!("target: {}", target);
    if target.is_empty() {
        1
    } else {
        available[&target.as_bytes()[0]]
            .iter()
            .map(|prefix| {
                if !target.starts_with(prefix) {
                    0
                } else {
                    count(available, &target[prefix.len()..])
                }
            })
            .sum::<u64>()
    }
}

fn part1() -> usize {
    let (available, targets) = parse();
    let p_available = available.iter().map(|&s| (s.as_bytes()[0], s)).fold(
        HashMap::from([
            (b'u', vec!["u"]),
            (b'g', vec!["g"]),
            (b'b', vec!["b"]),
            (b'r', vec!["r"]),
        ]),
        |mut acc, (b, prefix)| {
            if prefix.contains('w') {
                acc.entry(b).or_insert(vec![]).push(prefix);
            }
            acc
        },
    );
    targets
        .into_iter()
        .inspect(|raw| println!("raw: {}", raw))
        .filter(|&target| is_buildable(&p_available, target))
        .count()
}

fn main() {
    println!("Part 1: {}", part1());
}
