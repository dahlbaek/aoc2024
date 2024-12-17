use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

const PUZZLE: &[u8] = include_bytes!("puzzle");
const DIM: isize = 141;

type Position = (isize, isize);

#[derive(Copy, Clone, Debug, Ord, PartialOrd, PartialEq, Eq, Hash)]
enum Direction {
    East,
    North,
    West,
    South,
}

#[derive(Debug, Ord, PartialEq, PartialOrd, Eq, Hash)]
struct Vertex {
    score: u64,
    pos: Position,
    dir: Direction,
}

impl Vertex {
    fn forward(&self) -> Vertex {
        let pos = match self.dir {
            Direction::East => (self.pos.0, self.pos.1 + 1),
            Direction::North => (self.pos.0 - 1, self.pos.1),
            Direction::West => (self.pos.0, self.pos.1 - 1),
            Direction::South => (self.pos.0 + 1, self.pos.1),
        };
        Vertex {
            pos,
            dir: self.dir,
            score: self.score + 1,
        }
    }

    fn left(&self) -> Vertex {
        let dir = match self.dir {
            Direction::East => Direction::North,
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
        };
        Vertex {
            pos: self.pos,
            dir,
            score: self.score + 1000,
        }
    }

    fn right(&self) -> Vertex {
        let dir = match self.dir {
            Direction::East => Direction::South,
            Direction::North => Direction::East,
            Direction::West => Direction::North,
            Direction::South => Direction::West,
        };
        Vertex {
            pos: self.pos,
            dir,
            score: self.score + 1000,
        }
    }

    fn neighbors(&self) -> impl Iterator<Item = Vertex> {
        [self.forward(), self.left(), self.right()]
            .into_iter()
            .filter(|v| get(v.pos) != b'#')
    }
}

fn as_usize(i: isize) -> usize {
    i.try_into().unwrap()
}

fn get((row, col): Position) -> u8 {
    PUZZLE[as_usize(col + (DIM + 1) * row)]
}

fn as_position(u: usize) -> Position {
    let i: isize = u.try_into().unwrap();
    (i / (DIM + 1), i % (DIM + 1))
}

fn dijkstra() -> Vec<(Vertex, Vec<usize>)> {
    let start_vertex = Vertex {
        pos: as_position(PUZZLE.iter().position(|&b| b == b'S').unwrap()),
        dir: Direction::East,
        score: 0,
    };
    let mut seen = BinaryHeap::from([Reverse((start_vertex, None))]);
    let mut fixed: Vec<(Vertex, Vec<usize>)> = Vec::new();
    while let Some((vertex, previous_vertex_index)) = seen.pop().map(|r| r.0) {
        let fixed_elem = fixed
            .iter_mut()
            .find(|(v, _)| v.pos == vertex.pos && v.dir == vertex.dir);
        match fixed_elem {
            None => {
                vertex
                    .neighbors()
                    .for_each(|neighbor| seen.push(Reverse((neighbor, Some(fixed.len())))));
                fixed.push((vertex, previous_vertex_index.into_iter().collect()));
            }
            Some((v, previous_indices)) if v.score >= vertex.score => {
                previous_indices.extend(&previous_vertex_index);
            }
            Some(_) => {}
        }
    }
    fixed
}

fn extract_tiles(
    indices: impl Iterator<Item = usize>,
    shortest: &[(Vertex, Vec<usize>)],
) -> HashSet<Position> {
    let mut vertices = HashSet::new();
    let mut stack = Vec::new();
    for index in indices {
        stack.push(index);
        while let Some(index) = stack.pop() {
            let vertex = &shortest[index].0;
            if !vertices.contains(&vertex) {
                vertices.insert(vertex);
                stack.extend(&shortest[index].1);
            }
        }
    }
    vertices.into_iter().map(|v| v.pos).collect()
}

fn main() {
    let shortest = dijkstra();
    let part1 = shortest
        .iter()
        .filter(|(v, _)| get(v.pos) == b'E')
        .map(|(v, _)| v.score)
        .min()
        .unwrap();
    println!("Part 1: {}", part1);

    let indices = shortest
        .iter()
        .enumerate()
        .filter(|(_, (v, _))| get(v.pos) == b'E' && v.score == part1)
        .map(|(index, _)| index);
    let part2 = extract_tiles(indices, &shortest).len();
    println!("Part 2: {}", part2);
}
