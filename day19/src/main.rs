use std::{array::from_fn, cmp::min, collections::HashSet};

const PUZZLE: &str = include_str!("puzzle");
const MAX_AVAILABLE_LEN: usize = 8;

fn parse() -> (HashSet<&'static str>, Vec<&'static str>) {
    let mut lines = PUZZLE.trim().lines();
    let available = lines.next().unwrap().split(", ").collect();
    assert_eq!(lines.next().unwrap(), "");
    let targets = lines.collect();
    (available, targets)
}

fn is_buildable_small(available: &[HashSet<&str>; MAX_AVAILABLE_LEN], target: &str) -> bool {
    target.is_empty()
        || (1..=min(target.len(), MAX_AVAILABLE_LEN))
            .filter(|&len| available[len - 1].contains(&target[..len]))
            .any(|len| is_buildable_small(available, &target[len..]))
}

fn is_buildable(available: &[HashSet<&str>; MAX_AVAILABLE_LEN], target: &str) -> bool {
    if target.len() < 2 * MAX_AVAILABLE_LEN {
        is_buildable_small(available, target)
    } else {
        (1..=MAX_AVAILABLE_LEN)
            .flat_map(|len| {
                let midsection_start = target.len() / 2 - len + 1;
                (midsection_start..=target.len() / 2).map(move |index| (index, len))
            })
            .filter(|&(index, len)| available[len - 1].contains(&target[index..index + len]))
            .any(|(index, len)| {
                is_buildable(available, &target[..index + len])
                    && is_buildable(available, &target[index + len..])
            })
    }
}

fn count_small(available: &[HashSet<&str>; MAX_AVAILABLE_LEN], target: &str) -> u64 {
    if target.is_empty() {
        1
    } else {
        (1..=min(target.len(), MAX_AVAILABLE_LEN))
            .filter(|&len| available[len - 1].contains(&target[..len]))
            .map(|j| count_small(available, &target[j..]))
            .sum::<u64>()
    }
}

fn count(available: &[HashSet<&str>; MAX_AVAILABLE_LEN], target: &str) -> u64 {
    if target.len() < 2 * MAX_AVAILABLE_LEN {
        count_small(available, target)
    } else {
        (1..=MAX_AVAILABLE_LEN)
            .flat_map(|len| {
                let midsection_start = target.len() / 2 - len + 1;
                (midsection_start..=target.len() / 2).map(move |index| (index, len))
            })
            .filter(move |&(index, len)| available[len - 1].contains(&target[index..index + len]))
            .map(move |(index, len)| {
                count(available, &target[..index]) * count(available, &target[index + len..])
            })
            .sum()
    }
}

fn main() {
    let (available, targets) = parse();

    let mut available_partitioned = from_fn::<HashSet<_>, MAX_AVAILABLE_LEN, _>(|_| HashSet::new());
    for a in available {
        available_partitioned[a.len() - 1].insert(a);
    }

    let part1 = targets
        .iter()
        .filter(|target| is_buildable(&available_partitioned, target))
        .count();
    println!("Part 1: {}", part1);

    let part2 = targets
        .iter()
        .map(|target| count(&available_partitioned, target))
        .sum::<u64>();
    println!("Part 2: {}", part2);
}
