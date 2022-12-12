use std::{collections::VecDeque, path::Path};

use itertools::iproduct;
use rayon::prelude::{ParallelBridge, ParallelIterator};

mod input;

type Grid = Vec<Vec<char>>;

pub fn solve(path: &Path) {
    let grid = input::read_values(path)
        .unwrap_or_else(|e| panic!("Failed to read input file for {path:?} due to {e}"));
    let start = find_char(&grid, 'S').expect("Start S not found");
    let end = find_char(&grid, 'E').expect("End E not found");

    let dist = bfs(&grid, start, end).expect("No dist found for Hill Climbing Part 1");
    println!("Hill Climbing Algorithm Part 1: {dist}");

    let m = grid.len();
    let n = grid.first().expect("Empty Grid").len();

    let min = iproduct!(0..m, 0..n)
        .into_iter()
        .par_bridge()
        .filter(|(i, j)| grid[*i][*j] == 'a')
        .filter_map(|(i, j)| bfs(&grid, (i, j), end))
        .min()
        .expect("No min found for Hill Climbing Part 2");

    println!("Hill Climbing Algorithm Part 2: {min}");
}

fn find_char(g: &Grid, s: char) -> Option<(usize, usize)> {
    for (i, v) in g.iter().enumerate() {
        for (j, c) in v.iter().enumerate() {
            if *c == s {
                return Some((i, j));
            }
        }
    }
    None
}

fn bfs(grid: &Grid, start: (usize, usize), end: (usize, usize)) -> Option<i32> {
    let m = grid.len();
    let n = grid.first().expect("Empty Grid").len();

    let mut visited = vec![vec![false; n]; m];
    let mut dist = vec![vec![i32::MAX; n]; m];

    let (si, sj) = start;
    let (ei, ej) = end;

    let mut q = VecDeque::from([(si, sj)]);
    visited[si][sj] = true;
    dist[si][sj] = 0;

    while let Some((i, j)) = q.pop_front() {
        let cur: char = grid[i][j];
        let d = dist[i][j];
        for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let (i, j) = (i as i32 + di, j as i32 + dj);

            if 0 <= i && i < m as i32 && 0 <= j && j < n as i32 {
                let (i, j) = (i as usize, j as usize);
                let c = grid[i][j];

                let valid = match (cur, c) {
                    ('S', _) => true,
                    ('z', 'E') => true,
                    (_, 'E') => false,
                    (a, b) => (b as u8) <= (a as u8) + 1,
                };

                if !visited[i][j] && valid {
                    visited[i][j] = true;
                    dist[i][j] = d + 1;
                    q.push_back((i, j));
                }
            }
        }
    }

    if visited[ei][ej] {
        Some(dist[ei][ej])
    } else {
        None
    }
}
