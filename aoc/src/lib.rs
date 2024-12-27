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
