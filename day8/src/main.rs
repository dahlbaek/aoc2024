use std::collections::HashSet;

const PUZZLE: &[u8] = include_bytes!("puzzle");
const DIM: usize = 50;

fn index_of(raw_index: usize) -> (usize, usize) {
    (raw_index / (DIM + 1), raw_index % (DIM + 1))
}

fn insert_antinode(
    set: &mut HashSet<(usize, usize)>,
    (first_row, first_col): (usize, usize),
    (second_row, second_col): (usize, usize),
) -> Option<(usize, usize)> {
    if second_row <= 2 * first_row
        && 2 * first_row < DIM + second_row
        && second_col <= 2 * first_col
        && 2 * first_col < DIM + second_col
    {
        set.insert((2 * first_row - second_row, 2 * first_col - second_col));
        Some((2 * first_row - second_row, 2 * first_col - second_col))
    } else {
        None
    }
}

fn insert_antinodes(
    set: &mut HashSet<(usize, usize)>,
    mut first_index: (usize, usize),
    mut second_index: (usize, usize),
) {
    set.insert(first_index);
    set.insert(second_index);
    while let Some(next_index) = insert_antinode(set, first_index, second_index) {
        second_index = first_index;
        first_index = next_index;
    }
    while let Some(next_index) = insert_antinode(set, second_index, first_index) {
        first_index = second_index;
        second_index = next_index;
    }
}

fn main() {
    let antennae = PUZZLE
        .iter()
        .enumerate()
        .filter(|(_, &b)| b != b'.' && b != b'\n')
        .map(|(raw_index, b)| (index_of(raw_index), b))
        .collect::<Vec<_>>();

    let mut part1 = HashSet::new();
    for (first, (first_index, first_b)) in antennae.iter().enumerate() {
        for (second_index, second_b) in antennae.iter().skip(first + 1) {
            if first_b == second_b {
                insert_antinode(&mut part1, *first_index, *second_index);
                insert_antinode(&mut part1, *second_index, *first_index);
            }
        }
    }
    println!("Part 1: {}", part1.len());

    let mut part2 = HashSet::new();
    for (first, (first_index, first_b)) in antennae.iter().enumerate() {
        for (second_index, second_b) in antennae.iter().skip(first + 1) {
            if first_b == second_b {
                insert_antinodes(&mut part2, *first_index, *second_index);
            }
        }
    }
    println!("Part 2: {}", part2.len());
}
