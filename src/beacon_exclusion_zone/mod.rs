use std::{ops::Range, path::Path};

use itertools::Itertools;

mod input;

pub fn solve(path: &Path) {
    let values = input::read_values(path)
        .unwrap_or_else(|e| panic!("Failed to read input file for {path:?} due to {e}"));

    let mut sensors = vec![];
    for (s, b) in values {
        sensors.push(Sensor::new(s, Beacon::new(b)))
    }

    let no_beacon = sensors
        .iter()
        .flat_map(|s| s.beacons_x_not_in_row(2000000))
        .unique()
        .count();

    println!("Beacon Exclusion Zone Part 1: {no_beacon}");

    // let set = DashSet::new();
    // iproduct!(0..=4000000, 0..=4000000)
    //     .into_iter()
    //     .par_bridge()
    //     .for_each(|(x, y)| {
    //         if sensors.iter().all(|s| s.is_beacon_possible_at(Beacon { x, y })) {
    //             set.insert((x, y));
    //         }
    //     });
    // --------- Too Slow 

    let limit = 4000000i64;

    (0..=limit)
        .into_iter()
        .map(|y| {
            (
                y,
                sensors
                    .iter()
                    .map(|s| s.beacons_x_not_in_row(y as i32))
                    .collect::<Vec<_>>(),
            )
        })
        .for_each(|(y, v)| {
            let v = get_remainder_range(v, 0);
            if !v.is_empty() {
                let (start, _) = v[0];
                let x = (start + 1) as i64;
                println!("Beacon Exclusion Zone Part 2: {}", x * limit + y);
            }
        });
}

fn get_remainder_range(ranges: Vec<Range<i32>>, st: i32) -> Vec<(i32, i32)> {
    let mut ranges: Vec<_> = ranges
        .into_iter()
        .map(|Range { start, end }| (start, end))
        .collect();
    ranges.sort_unstable();

    let mut uncover: Vec<(i32, i32)> = vec![];
    let mut ce = st;

    for (s, e) in ranges {
        if s > ce + 1 {
            uncover.push((ce, s));
        }
        ce = ce.max(e);
    }

    uncover
}

#[derive(Clone, Copy)]
struct Sensor {
    x: i32,
    y: i32,
    beacon: Beacon,
}

impl Sensor {
    pub fn new((x, y): (i32, i32), beacon: Beacon) -> Self {
        Self { x, y, beacon }
    }

    fn get_manhattan_distance(&self, b: Beacon) -> u32 {
        self.x.abs_diff(b.x) + self.y.abs_diff(b.y)
    }

    // fn is_beacon_possible_at(&self, b: Beacon) -> bool {
    //     self.get_manhattan_distance(b) > self.get_manhattan_distance(self.beacon)
    // }

    fn beacons_x_not_in_row(&self, y: i32) -> Range<i32> {
        let mhd = self.get_manhattan_distance(self.beacon);

        let dist_row = self.y.abs_diff(y);

        if dist_row > mhd {
            0..0
        } else {
            let delta = mhd - dist_row;
            (self.x - delta as i32)..(self.x + delta as i32)
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Beacon {
    pub x: i32,
    pub y: i32,
}

impl Beacon {
    pub fn new((x, y): (i32, i32)) -> Self {
        Beacon { x, y }
    }
}
