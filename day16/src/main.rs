use std::{cmp::Reverse, collections::BinaryHeap};

const PUZZLE: &[u8] = include_bytes!("puzzle");
const DIM: isize = 141;

type Position = (isize, isize);

#[derive(Copy, Clone, Debug, Ord, PartialOrd, PartialEq, Eq)]
enum Direction {
    East,
    North,
    West,
    South,
}

#[derive(Debug, Ord, PartialEq, PartialOrd, Eq)]
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

    fn is_valid(&self) -> bool {
        get(self.pos) != b'#'
    }

    fn neighbors(&self) -> impl Iterator<Item = Vertex> {
        [self.forward(), self.left(), self.right()]
            .into_iter()
            .filter(Vertex::is_valid)
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

fn dijkstra() -> Vec<Vertex> {
    let start_vertex = Vertex {
        pos: as_position(PUZZLE.iter().position(|&b| b == b'S').unwrap()),
        dir: Direction::East,
        score: 0,
    };
    let mut seen = BinaryHeap::from([Reverse(start_vertex)]);
    let mut fixed: Vec<Vertex> = Vec::new();
    while let Some(vertex) = seen.pop().map(|r| r.0) {
        let already_fixed = fixed
            .iter()
            .any(|v| v.pos == vertex.pos && v.dir == vertex.dir);
        if !already_fixed {
            vertex
                .neighbors()
                .for_each(|neighbor| seen.push(Reverse(neighbor)));
            fixed.push(vertex);
        }
    }
    fixed
}

fn main() {
    let part1 = dijkstra()
        .into_iter()
        .filter(|v| get(v.pos) == b'E')
        .map(|v| v.score)
        .min()
        .unwrap();
    println!("Part 1: {}", part1);
}
