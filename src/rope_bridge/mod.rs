use std::{collections::HashSet, path::Path};

mod input;
mod rope;

use rope::Rope;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn solve(path: &Path) {
    let values = input::read_values(path)
        .unwrap_or_else(|e| panic!("Failed to read input file for {path:?} due to {e}"));

    let mut rope_2: Rope<2> = Rope::new();
    let mut rope_10: Rope<10> = Rope::new();

    let mut set_2 = HashSet::new();
    let mut set_10 = HashSet::new();
    for d in values {
        rope_2.move_head(d);
        set_2.insert(rope_2.tail());
        rope_10.move_head(d);
        set_10.insert(rope_10.tail());
    }

    println!("Rope Bridge Part 1 {}", set_2.len());
    println!("Rope Bridge Part 2 {}", set_10.len());
}
