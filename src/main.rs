use std::collections::HashSet;
use raylib::prelude::*;

#[derive(PartialEq, Eq, Hash)]
pub struct Coord { x: i64, y: i64 }

impl Coord {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

pub struct Grid {
    live: HashSet<Coord>,
}

impl Grid {
    pub fn new() -> Self {
        Self { live: HashSet::new() }
    }

    pub fn spawn(&mut self, pos: Coord) {
        self.live.insert(pos);
    }

    pub fn kill(&mut self, pos: Coord) {
        self.live.remove(&pos);
    }

    pub fn is_alive(&self, pos: &Coord) -> bool {
        self.live.contains(pos)
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .build();

    let mut grid = Grid::new();
    grid.spawn(Coord::new(1,0));
    grid.spawn(Coord::new(2,0));
    grid.spawn(Coord::new(0,1));
    grid.spawn(Coord::new(1,1));
    grid.spawn(Coord::new(1,2));

    for y in 0..20 as i64 {
        for x in 0..20 as i64 {
            let pos = Coord::new(x - 5, y - 5);
            if grid.is_alive(&pos) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        for y in 0..20 {
            for x in 0..20 {
                let pos = Coord::new(x - 5, y - 5);
                if grid.is_alive(&pos) {
                    d.draw_rectangle((x as i32 - 5) * 20, (y as i32 - 5) * 20, 20, 20, Color::new(0, 0, 0, 255));
                } else {
                }
            }
        }
    }
}
