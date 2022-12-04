use std::path::Path;

mod input;

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub start: u32,
    pub stop: u32
}

impl Interval {
    pub fn new(start: u32, stop: u32) -> Self {
        Self {
            start,
            stop
        }
    }

    pub fn contains(&self, other: Interval) -> bool {
        self.start <= other.start && other.stop <= self.stop
    }

    pub fn overlaps(&self, other: Interval) -> bool {
        self.contains(other) || other.contains(*self)
        || (self.start <= other.start && other.start <= self.stop)
        || (self.start <= other.stop && other.stop <= self.stop)
    }
}

pub fn solve(path: &Path) {
    let values = input::read_values(path).unwrap_or_else(|e| panic!("Failed with {e}"));

    let mut contained = 0;
    for (a, b) in values.clone() {
        if a.contains(b) || b.contains(a) {
            contained += 1;
        }
    }

    println!("Camp Cleanup Part 1 {contained}");

    let mut contained = 0;
    for (a, b) in values {
        if a.overlaps(b) {
            contained += 1;
        }
    }

    println!("Camp Cleanup Part 2 {contained}");
}
