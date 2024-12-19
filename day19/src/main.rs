use std::collections::HashSet;

const PUZZLE: &str = include_str!("puzzle");

fn parse() -> (HashSet<&'static str>, Vec<&'static str>) {
    let mut lines = PUZZLE.trim().lines();
    let available = lines.next().unwrap().split(", ").collect();
    assert_eq!(lines.next().unwrap(), "");
    let targets = lines.collect();
    (available, targets)
}

fn is_buildable_small(available: &HashSet<&str>, target: &str) -> bool {
    target.is_empty()
        || (1..=target.len()).any(|j| {
            available.contains(&target[..j]) && is_buildable_small(available, &target[j..])
        })
}

fn is_buildable(available: &HashSet<&str>, target: &str) -> bool {
    if target.len() < 16 {
        is_buildable_small(available, target)
    } else {
        available.iter().any(|s| {
            let midsection_start = target.len() / 2 - 4;
            target[midsection_start..target.len() / 2 + 4]
                .find(s)
                .is_some_and(|index| {
                    is_buildable(available, &target[..midsection_start + index])
                        && is_buildable(available, &target[midsection_start + index + s.len()..])
                })
        })
    }
}

fn count_small(available: &HashSet<&str>, target: &str) -> u64 {
    if target.is_empty() {
        1
    } else {
        (1..=target.len())
            .filter(|&j| available.contains(&target[..j]))
            .map(|j| count_small(available, &target[j..]))
            .sum::<u64>()
    }
}

fn count(available: &HashSet<&str>, target: &str) -> u64 {
    if target.len() < 16 {
        count_small(available, target)
    } else {
        available
            .iter()
            .flat_map(|&s| {
                let midsection_start = target.len() / 2 - s.len() + 1;
                (midsection_start..=target.len() / 2)
                    .filter(move |&j| &target[j..j + s.len()] == s)
                    .map(move |j| {
                        count(available, &target[..j]) * count(available, &target[j + s.len()..])
                    })
            })
            .sum()
    }
}

fn main() {
    let (available, targets) = parse();

    let part1 = targets
        .iter()
        .filter(|target| is_buildable(&available, target))
        .count();
    println!("Part 1: {}", part1);

    let part2 = targets
        .iter()
        .map(|target| count(&available, target))
        .sum::<u64>();
    println!("Part 2: {}", part2);
}
