use std::{
    collections::{HashSet, VecDeque},
    path::Path,
};

use dashmap::DashSet;
use itertools::{Itertools, MinMaxResult};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

mod cube;
mod input;

use cube::Cube;

pub fn solve(path: &Path) {
    let cubes = input::read_values(path)
        .unwrap_or_else(|e| panic!("Failed to read input file for {path:?} due to {e}"));
    let cubes: DashSet<Cube> = cubes.into_iter().collect();

    let visible_faces: Vec<usize> = cubes
        .par_iter()
        .map(|c| c.neighbors().iter().filter(|n| !cubes.contains(n)).count())
        .collect();

    println!(
        "Boiling Boulders Part 1: {}",
        visible_faces.iter().sum::<usize>()
    );

    let MinMaxResult::MinMax(min_x, max_x) = cubes.iter().map(|c| c.x).minmax() else {unreachable!("Could not find MinMax_X")};
    let MinMaxResult::MinMax(min_y, max_y) = cubes.iter().map(|c| c.y).minmax() else {unreachable!("Could not find MinMax_Y")};
    let MinMaxResult::MinMax(min_z, max_z) = cubes.iter().map(|c| c.z).minmax() else {unreachable!("Could not find MinMax_Z")};

    let start_cube = Cube {
        x: min_x - 1,
        y: min_y - 1,
        z: min_z - 1,
    };

    let in_bounded_region = |c: &Cube| -> bool {
        c.x >= min_x - 2
            && c.x <= max_x + 2
            && c.y >= min_y - 2
            && c.y <= max_y + 2
            && c.z >= min_z - 2
            && c.z <= max_z + 2
    };

    let mut visited = HashSet::new();
    let mut surfaces = HashSet::new();

    visited.insert(start_cube);
    let mut q = VecDeque::from([start_cube]);
    while let Some(air_cube) = q.pop_front() {
        for n in air_cube.neighbors().into_iter().filter(in_bounded_region) {
            if cubes.contains(&n) {
                surfaces.insert((air_cube, n));
            } else if !visited.contains(&n) {
                visited.insert(n);
                q.push_back(n);
            }
        }
    }

    println!("Boiling Boulders Part 2: {}", surfaces.len());
}
