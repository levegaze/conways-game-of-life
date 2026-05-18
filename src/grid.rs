use crate::hash::{BuildPosHasher, LiveSet, PosMap};

/// A cell coordinate in the grid
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Position(pub i32, pub i32);

impl Position {


    #[inline]
    pub fn new(x: i32, y: i32) -> Self {
        Position(x, y)
    }

    #[inline]
    pub fn x(&self) -> i32 {
        self.0
    }

    #[inline]
    pub fn y(&self) -> i32 {
        self.1
    }
}

/// 8 neighboring cell offsets (pre-computed to avoid repeated creation)
pub const NEIGHBOR_OFFSETS: [(i32, i32); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];

/// Grid properties and utilities for Game of Life simulation
pub struct Grid {
    pub width: i32,         // Grid width in cells
    pub height: i32,        // Grid height in cells
    pub wrap_world: bool,    // Whether cells wrap around edges
}

impl Grid {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            wrap_world: false,
        }
    }

    #[inline]
    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        (0..self.width).contains(&x) && (0..self.height).contains(&y)
    }

    #[inline]
    pub fn wrap(&self, x: i32, y: i32) -> Position {
        let mut nx = x % self.width;
        let mut ny = y % self.height;
        if nx < 0 { nx += self.width; }
        if ny < 0 { ny += self.height; }
        Position::new(nx, ny)
    }

    /// Calculate next generation of cells.
    ///
    /// Bit 7 (`0x80`) of each count entry marks a currently-live cell,
    /// so we don't need a second `live.contains()` lookup per output cell.
    pub fn next_generation(&self, live: &LiveSet) -> LiveSet {
        let mut counts: PosMap<u8> =
            PosMap::with_capacity_and_hasher(live.len() * 4 + 8, BuildPosHasher::default());

        for &cell in live {
            *counts.entry(cell).or_insert(0) |= 0x80;
            for (dx, dy) in NEIGHBOR_OFFSETS {
                let p = if self.wrap_world {
                    self.wrap(cell.x() + dx, cell.y() + dy)
                } else {
                    Position::new(cell.x() + dx, cell.y() + dy)
                };
                if self.wrap_world || self.in_bounds(p.x(), p.y()) {
                    *counts.entry(p).or_insert(0) += 1;
                }
            }
        }

        // B3/S23: birth at 3 neighbors, survival at 2 or 3.
        let mut next = LiveSet::with_capacity_and_hasher(live.len(), BuildPosHasher::default());
        for (pos, c) in counts {
            let n = c & 0x7f;
            let alive = c & 0x80 != 0;
            if n == 3 || (alive && n == 2) {
                next.insert(pos);
            }
        }

        next
    }
}