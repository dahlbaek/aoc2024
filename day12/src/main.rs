use std::{collections::HashSet, iter::from_fn};

const PUZZLE: &[u8] = include_bytes!("puzzle");
const DIM: usize = 140;

type Position = (usize, usize);
type Positions = HashSet<Position>;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Side {
    East,
    North,
    West,
    South,
}

fn get((row, col): Position) -> u8 {
    PUZZLE[col + (DIM + 1) * row]
}

fn neighbors((row, col): Position) -> impl Iterator<Item = Position> {
    [
        row.checked_sub(1).map(|r| (r, col)),
        Some((row + 1, col)).filter(|&(r, _)| r < DIM),
        col.checked_sub(1).map(|c| (row, c)),
        Some((row, col + 1)).filter(|&(_, c)| c < DIM),
    ]
    .into_iter()
    .flatten()
}

fn get_unvisited(visited: &Positions, seen: &Positions) -> Option<Position> {
    seen.iter().filter(|p| !visited.contains(p)).next().cloned()
}

fn region_chunks() -> impl FnMut() -> Option<Positions> {
    let mut visited = Positions::new();
    let mut points = (0..DIM).flat_map(|row| (0..DIM).map(move |col| (row, col)));
    move || {
        points.find(|p| !visited.contains(p)).map(|position| {
            let plant = get(position);
            let mut positions = Positions::from([position]);
            while let Some(position) = get_unvisited(&visited, &positions) {
                visited.insert(position);
                for neighbor in neighbors(position) {
                    if get(neighbor) == plant {
                        positions.insert(neighbor);
                    }
                }
            }
            positions
        })
    }
}

fn get_perimeter(region: &Positions) -> usize {
    region
        .iter()
        .map(|&position| 4 - neighbors(position).filter(|n| region.contains(n)).count())
        .sum::<usize>()
}

fn pop_adjacent(
    side: &HashSet<(Position, Side)>,
    separate_sides: &mut Vec<(Position, Side)>,
) -> Option<(Position, Side)> {
    separate_sides
        .iter()
        .position(|s| match s.1 {
            Side::East | Side::West => {
                (s.0 .0 > 0 && side.contains(&((s.0 .0 - 1, s.0 .1), s.1)))
                    || side.contains(&((s.0 .0 + 1, s.0 .1), s.1))
            }
            Side::North | Side::South => {
                (s.0 .1 > 0 && side.contains(&((s.0 .0, s.0 .1 - 1), s.1)))
                    || side.contains(&((s.0 .0, s.0 .1 + 1), s.1))
            }
        })
        .map(|index| separate_sides.remove(index))
}

fn get_sides(positions: &Positions) -> usize {
    let mut separate_sides = positions
        .iter()
        .flat_map(|p| {
            let mut storage = Vec::new();
            if p.0 == 0 || !positions.contains(&(p.0 - 1, p.1)) {
                storage.push((*p, Side::North));
            }
            if p.0 == DIM - 1 || !positions.contains(&(p.0 + 1, p.1)) {
                storage.push((*p, Side::South));
            }
            if p.1 == 0 || !positions.contains(&(p.0, p.1 - 1)) {
                storage.push((*p, Side::West));
            }
            if p.1 == DIM - 1 || !positions.contains(&(p.0, p.1 + 1)) {
                storage.push((*p, Side::East));
            }
            storage
        })
        .collect::<Vec<_>>();

    let mut connected_sides = 0;
    while let Some(separate_side) = separate_sides.pop() {
        let mut connected_side = HashSet::new();
        connected_side.insert(separate_side);
        while let Some(adjacent_side) = pop_adjacent(&connected_side, &mut separate_sides) {
            connected_side.insert(adjacent_side);
        }
        connected_sides += 1;
    }
    connected_sides
}

fn main() {
    let regions = from_fn(region_chunks()).collect::<Vec<_>>();

    let part1 = regions
        .iter()
        .map(|region| get_perimeter(region) * region.len())
        .sum::<usize>();
    println!("Part 1: {}", part1);

    let part2 = regions
        .iter()
        .map(|region| get_sides(&region) * region.len())
        .sum::<usize>();
    println!("Part 2: {}", part2)
}
