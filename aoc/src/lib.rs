use std::{borrow::Borrow, collections::HashMap, hash::Hash};

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

pub enum Direction {
    East,
    North,
    West,
    South,
}

pub fn tc_dist(pos1: Position, pos2: Position) -> i64 {
    (pos1.x - pos2.x).abs() + (pos1.y - pos2.y).abs()
}

pub struct Grid {
    xmin: i64,
    xmax: i64,
    ymin: i64,
    ymax: i64,
}

impl Grid {
    pub const fn new(xdim: i64, ydim: i64) -> Grid {
        Grid {
            xmax: xdim - 1,
            ymax: ydim - 1,
            xmin: 0,
            ymin: 0,
        }
    }

    pub fn neighbours<'a>(
        &'a self,
        position: Position,
    ) -> impl Iterator<Item = (Direction, Position)> + 'a {
        let Position { x, y } = position;
        [
            (Direction::East, Position { x: x + 1, y }),
            (Direction::North, Position { x, y: y - 1 }),
            (Direction::West, Position { x: x - 1, y }),
            (Direction::South, Position { x, y: y + 1 }),
        ]
        .into_iter()
        .filter(|(_, pos)| {
            pos.x >= self.xmin && pos.x <= self.xmax && pos.y >= self.ymin && pos.y <= self.ymax
        })
    }
}

pub struct Cache<K, V, Q: ?Sized, C> {
    inner: HashMap<K, V>,
    f: fn(&mut Self, &Q, &C) -> V,
}

impl<K, V, Q, C> Cache<K, V, Q, C>
where
    Q: ?Sized,
    Q: Eq + Hash,
    K: Eq + Hash,
    Q: ToOwned<Owned = K>,
    K: Borrow<Q>,
    V: Clone,
{
    pub fn new(f: fn(&mut Self, &Q, &C) -> V) -> Cache<K, V, Q, C> {
        let inner = HashMap::new();
        Cache { inner, f }
    }

    pub fn get_or_compute(&mut self, q: &Q, c: &C) -> V {
        match self.inner.get(q) {
            Some(v) => v.clone(),
            None => {
                let v = (self.f)(self, q, c);
                self.inner.insert(q.to_owned(), v.clone());
                v
            }
        }
    }
}
