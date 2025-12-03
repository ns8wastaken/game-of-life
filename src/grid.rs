use std::collections::{HashMap, HashSet};
use crate::coord::Coord;

pub struct Grid {
    live: HashSet<Coord>,
}

impl Grid {
    pub fn new() -> Self {
        Self { live: HashSet::new() }
    }

    pub fn is_alive(&self, pos: &Coord) -> bool {
        self.live.contains(pos)
    }

    pub fn spawn(&mut self, pos: Coord) {
        self.live.insert(pos);
    }

    pub fn kill(&mut self, pos: Coord) {
        self.live.remove(&pos);
    }

    #[inline]
    fn get_neighbors(pos: &Coord) -> [Coord; 8] {
        [
            Coord::new(pos.x - 1, pos.y - 1),
            Coord::new(pos.x, pos.y - 1),
            Coord::new(pos.x + 1, pos.y - 1),
            Coord::new(pos.x - 1, pos.y),
            Coord::new(pos.x + 1, pos.y),
            Coord::new(pos.x - 1, pos.y + 1),
            Coord::new(pos.x, pos.y + 1),
            Coord::new(pos.x + 1, pos.y + 1),
        ]
    }

    pub fn step(&mut self) {
        let mut count = HashMap::<Coord, u8>::new();

        for pos in &self.live {
            for n in Self::get_neighbors(&pos) {
                *count.entry(n).or_default() += 1;
            }
        }

        let mut next = HashSet::new();
        for (cell, n) in count {
            if n == 3 || (n == 2 && self.is_alive(&cell)) {
                next.insert(cell);
            }
        }

        self.live = next
    }

    pub fn get_live(&self) -> &HashSet<Coord> {
        &self.live
    }
}
