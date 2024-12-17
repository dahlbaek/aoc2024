use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

type Int = isize;
type Position = (Int, Int);

const PUZZLE: &[u8] = include_bytes!("puzzle");
const DIM: Int = 141;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, PartialEq, Eq, Hash)]
enum Direction {
    East,
    North,
    West,
    South,
}

#[derive(Debug, Ord, PartialEq, PartialOrd, Eq, Hash, Clone)]
struct Vertex {
    pos: Position,
    dir: Direction,
}

struct Label {
    score: u64,
    previous: Vec<Vertex>,
}

impl Label {
    fn new(score: u64, previous: Vec<Vertex>) -> Label {
        Label { score, previous }
    }
}

impl Vertex {
    fn forward(&self) -> Vertex {
        let pos = match self.dir {
            Direction::East => (self.pos.0, self.pos.1 + 1),
            Direction::North => (self.pos.0 - 1, self.pos.1),
            Direction::West => (self.pos.0, self.pos.1 - 1),
            Direction::South => (self.pos.0 + 1, self.pos.1),
        };
        Vertex { pos, dir: self.dir }
    }

    fn left(&self) -> Vertex {
        let dir = match self.dir {
            Direction::East => Direction::North,
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
        };
        Vertex { pos: self.pos, dir }
    }

    fn right(&self) -> Vertex {
        let dir = match self.dir {
            Direction::East => Direction::South,
            Direction::North => Direction::East,
            Direction::West => Direction::North,
            Direction::South => Direction::West,
        };
        Vertex { pos: self.pos, dir }
    }

    fn neighbors(&self, score: u64) -> impl Iterator<Item = (u64, Vertex)> {
        [
            (score + 1, self.forward()),
            (score + 1000, self.left()),
            (score + 1000, self.right()),
        ]
        .into_iter()
        .filter(|(_, v)| get(v.pos) != b'#')
    }
}

fn into_usize(i: Int) -> usize {
    i.try_into().unwrap()
}

fn get((row, col): Position) -> u8 {
    PUZZLE[into_usize(col + (DIM + 1) * row)]
}

fn as_position(u: usize) -> Position {
    let i: Int = u.try_into().unwrap();
    (i / (DIM + 1), i % (DIM + 1))
}

fn dijkstra() -> HashMap<Vertex, Label> {
    let start_vertex = Vertex {
        pos: as_position(PUZZLE.iter().position(|&b| b == b'S').unwrap()),
        dir: Direction::East,
    };
    let start_label = Label::new(0, Vec::new());
    let mut heap = BinaryHeap::from([(Reverse(start_label.score), start_vertex.clone())]);
    let mut labels = HashMap::from([(start_vertex, start_label)]);
    while let Some((_, vertex)) = heap.pop() {
        let label = labels.get_mut(&vertex).unwrap();
        for (neighbor_score, neighbor) in vertex.neighbors(label.score) {
            match labels.get_mut(&neighbor) {
                Some(neighbor_label) if neighbor_label.score < neighbor_score => {}
                Some(neighbor_label) if neighbor_label.score == neighbor_score => {
                    if !neighbor_label.previous.contains(&vertex) {
                        neighbor_label.previous.push(vertex.clone());
                    }
                }
                Some(_) | None => {
                    let neighbor_label = Label::new(neighbor_score, vec![vertex.clone()]);
                    labels.insert(neighbor.clone(), neighbor_label);
                    heap.push((Reverse(neighbor_score), neighbor));
                }
            }
        }
    }
    labels
}

fn extract_tiles(
    start_vertices: impl Iterator<Item = Vertex>,
    shortest: &HashMap<Vertex, Label>,
) -> HashSet<Position> {
    let mut vertices = HashSet::new();
    let mut stack = Vec::new();
    for vertex in start_vertices {
        stack.push(vertex);
        while let Some(previous_vertex) = stack.pop() {
            if !vertices.contains(&previous_vertex) {
                stack.extend(shortest[&previous_vertex].previous.clone());
                vertices.insert(previous_vertex);
            }
        }
    }
    vertices.into_iter().map(|v| v.pos).collect()
}

fn main() {
    let shortest = dijkstra();
    let part1 = shortest
        .iter()
        .filter(|(vertex, _)| get(vertex.pos) == b'E')
        .map(|(_, label)| label.score)
        .min()
        .unwrap();
    println!("Part 1: {}", part1);

    let start_vertices = shortest
        .iter()
        .filter(|(vertex, label)| get(vertex.pos) == b'E' && label.score == part1)
        .map(|(vertex, _)| vertex)
        .cloned();
    let part2 = extract_tiles(start_vertices, &shortest).len();
    println!("Part 2: {}", part2);
}
