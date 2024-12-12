use std::{
    collections::{HashMap, HashSet},
    iter,
};

const PUZZLE: &[u8] = include_bytes!("puzzle");
const DIM: usize = 45;

type Scores = HashMap<(usize, usize), HashSet<(usize, usize)>>;
type Ratings = HashMap<(usize, usize), u64>;

fn get((col, row): (usize, usize)) -> u8 {
    PUZZLE[col + (DIM + 1) * row] - 48
}

fn get_highes_indices() -> impl Iterator<Item = (usize, usize)> {
    (0..DIM)
        .flat_map(|i| (0..DIM).map(move |j| (i, j)))
        .filter(move |&index| get(index) == 9)
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
    .filter(move |&next_index| get((col, row)) == 1 + get(next_index))
}

fn next_scores(scores: &Scores) -> Option<Scores> {
    let next = scores
        .iter()
        .flat_map(|(&k, v)| next_indices(k).map(move |next_index| (next_index, v)))
        .fold(Scores::new(), |mut new_state, (next_index, v)| {
            new_state.entry(next_index).or_default().extend(v);
            new_state
        });
    Some(next)
}

fn next_ratings(ratings: &Ratings) -> Option<Ratings> {
    let next = ratings
        .iter()
        .flat_map(|(&k, v)| next_indices(k).map(move |next_index| (next_index, v)))
        .fold(Ratings::new(), |mut new_state, (next_index, v)| {
            *new_state.entry(next_index).or_default() += v;
            new_state
        });
    Some(next)
}

fn main() {
    let first_scores = get_highes_indices().map(|index| (index, HashSet::from([index])));
    let part1 = iter::successors(Some(first_scores.collect()), next_scores)
        .nth(9)
        .unwrap()
        .values()
        .map(|s| s.len())
        .sum::<usize>();

    println!("Part 1: {}", part1);

    let first_ratings = get_highes_indices().map(|index| (index, 1));
    let part2 = iter::successors(Some(first_ratings.collect()), next_ratings)
        .nth(9)
        .unwrap()
        .values()
        .sum::<u64>();

    println!("Part 2: {}", part2);
}
