use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

const PUZZLE: &[u8] = include_bytes!("puzzle");
const DIM: usize = 45;

fn get((col, row): (usize, usize)) -> u8 {
    PUZZLE[col + (DIM + 1) * row] - 48
}

fn get_descending_indices() -> Vec<(usize, usize)> {
    let mut indices = (0..DIM)
        .flat_map(|i| (0..DIM).map(move |j| (i, j)))
        .collect::<Vec<_>>();
    indices.sort_unstable_by_key(|&index| Reverse(get(index)));
    indices
}

fn next_indices((col, row): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    [
        col.checked_sub(1).map(|c| (c, row)),
        Some((col + 1, row)).filter(|&(c, _)| c < DIM),
        row.checked_sub(1).map(|r| (col, r)),
        Some((col, row + 1)).filter(|&(_, r)| r < DIM),
    ]
    .into_iter()
    .flatten()
}

fn add_score(
    state: &mut HashMap<(usize, usize), HashSet<(usize, usize)>>,
    &index: &(usize, usize),
) -> Option<((usize, usize), usize)> {
    let height = get(index);
    let value = if height == 9 {
        HashSet::from([index])
    } else {
        next_indices(index)
            .filter(|&next_index| get(next_index) == height + 1)
            .fold(HashSet::new(), |mut value, next_index| {
                value.extend(&state[&next_index]);
                value
            })
    };
    let score = value.len();
    state.insert(index, value);
    Some((index, score))
}

fn add_rating(
    state: &mut HashMap<(usize, usize), u64>,
    &index: &(usize, usize),
) -> Option<((usize, usize), u64)> {
    let height = get(index);
    let value = if height == 9 {
        1
    } else {
        next_indices(index)
            .filter(|&next_index| get(next_index) == height + 1)
            .fold(0, |value, next_index| value + state[&next_index])
    };
    state.insert(index, value);
    Some((index, value))
}

fn main() {
    let indices = get_descending_indices();

    let part1 = indices
        .iter()
        .scan(HashMap::new(), add_score)
        .filter(|&(index, _)| get(index) == 0)
        .map(|(_, score)| score)
        .sum::<usize>();

    println!("Part 1: {}", part1);

    let part2 = indices
        .iter()
        .scan(HashMap::new(), add_rating)
        .filter(|&(index, _)| get(index) == 0)
        .map(|(_, rating)| rating)
        .sum::<u64>();

    println!("Part 2: {}", part2);
}
