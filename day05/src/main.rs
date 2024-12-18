use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

const PUZZLE: &str = include_str!("puzzle");

fn parse_list(list: &str) -> Vec<usize> {
    list.split(',').map(|n| n.parse().unwrap()).collect()
}

type Relations = HashMap<usize, HashSet<usize>>;

fn add_relation(mut comes_before: Relations, relation: &str) -> Relations {
    let (before, after) = relation.split_once('|').unwrap();
    comes_before
        .entry(after.parse().unwrap())
        .or_default()
        .insert(before.parse().unwrap());
    comes_before
}

fn list_is_sorted(comes_before: &Relations, numbers: &[usize]) -> bool {
    !numbers.iter().enumerate().any(|(idx, j)| {
        comes_before.get(j).map_or(false, |before| {
            numbers[idx + 1..]
                .iter()
                .any(|after| before.contains(after))
        })
    })
}

fn sort_list(comes_before: &Relations, mut numbers: Vec<usize>) -> Vec<usize> {
    numbers.sort_by(|i, j| {
        let sorted = |i, j| {
            comes_before
                .get(j)
                .map_or(false, |before| before.contains(i))
        };

        if sorted(i, j) {
            Ordering::Less
        } else if sorted(j, i) {
            Ordering::Greater
        } else {
            panic!()
        }
    });
    numbers
}

fn main() {
    let (relations, lists) = PUZZLE.trim().split_once("\n\n").unwrap();

    let comes_before = relations.lines().fold(HashMap::new(), add_relation);
    let lists_parsed = lists.lines().map(parse_list).collect::<Vec<_>>();

    let part1 = lists_parsed
        .iter()
        .filter(|numbers| list_is_sorted(&comes_before, numbers))
        .map(|numbers| numbers[numbers.len() / 2])
        .sum::<usize>();

    println!("Part 1: {}", part1);

    let part2 = lists_parsed
        .into_iter()
        .filter(|numbers| !list_is_sorted(&comes_before, numbers))
        .map(|numbers| sort_list(&comes_before, numbers))
        .map(|numbers| numbers[numbers.len() / 2])
        .sum::<usize>();

    println!("Part 2: {}", part2);
}
