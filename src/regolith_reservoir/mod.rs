use std::{char, collections::HashMap, path::Path};

use itertools::{Itertools, MinMaxResult::MinMax};

mod input;
mod particle;

use particle::Particle;

type Point = (i32, i32);

pub fn solve(path: &Path, visualize: bool) {
    let values = input::read_values(path)
        .unwrap_or_else(|e| panic!("Failed to read input file for {path:?} due to {e}"));

    let (_, y): (Vec<_>, Vec<_>) = values.iter().flatten().cloned().unzip();
    let Some(max_y) = y.into_iter().max() else {panic!("Did not get min_y and max_y")};

    let mut grid: HashMap<(i32, i32), Particle> = HashMap::new();

    for line in values {
        line.windows(2).for_each(|w| draw(w[0], w[1], &mut grid))
    }

    let sand_source = (0, 500);

    let num_rocks = grid.len();
    while let Some(p) = fall_sand(sand_source, &grid, max_y) {
        grid.insert(p, Particle::Sand);
    }

    let num_particles = grid.len();
    let num_added = num_particles - num_rocks;
    println!("Regolith Reservoir Part 1: {num_added}");

    while let Some(p) = fall_sand_floor(sand_source, &grid, max_y) {
        grid.insert(p, Particle::Sand);
        if p == sand_source {
            break;
        }
    }

    let num_particles = grid.len();
    let num_added = num_particles - num_rocks;
    println!("Regolith Reservoir Part 2: {num_added}");

    if visualize {
        let (y, x): (Vec<_>, Vec<_>) = grid.keys().cloned().unzip();
        let MinMax(min_x, max_x) = x.into_iter().minmax() else {panic!("MinMax x not found")};
        let MinMax(min_y, max_y) = y.into_iter().minmax() else {panic!("MinMax y not found")};
        for i in min_y..=max_y {
            for j in min_x..=max_x {
                print!(
                    "{}",
                    char::from(*grid.get(&(i, j)).unwrap_or(&Particle::Air))
                );
            }
            println!();
        }
    }
}

pub fn draw((a, b): Point, (c, d): Point, grid: &mut HashMap<Point, Particle>) {
    if a == c {
        (b.min(d)..=b.max(d)).for_each(|i| {
            grid.insert((i, a), Particle::Rock);
        });
    } else if b == d {
        (a.min(c)..=a.max(c)).for_each(|i| {
            grid.insert((b, i), Particle::Rock);
        });
    } else {
        panic!("Line not straigh ({a}, {b}) -> ({c}, {d})");
    }
}

pub fn fall_sand((y, x): Point, grid: &HashMap<Point, Particle>, max_y: i32) -> Option<Point> {
    if y >= max_y {
        return None;
    }

    let down = (y + 1, x);
    let left = (y + 1, x - 1);
    let right = (y + 1, x + 1);

    match grid.get(&down) {
        None => fall_sand(down, grid, max_y),
        _ => match grid.get(&left) {
            None => fall_sand(left, grid, max_y),
            _ => match grid.get(&right) {
                None => fall_sand(right, grid, max_y),
                _ => Some((y, x)),
            },
        },
    }
}

pub fn fall_sand_floor(
    (y, x): Point,
    grid: &HashMap<Point, Particle>,
    max_y: i32,
) -> Option<Point> {
    if y == max_y + 1 {
        return Some((y, x));
    }

    let down = (y + 1, x);
    let left = (y + 1, x - 1);
    let right = (y + 1, x + 1);

    match grid.get(&down) {
        None => fall_sand_floor(down, grid, max_y),
        _ => match grid.get(&left) {
            None => fall_sand_floor(left, grid, max_y),
            _ => match grid.get(&right) {
                None => fall_sand_floor(right, grid, max_y),
                _ => Some((y, x)),
            },
        },
    }
}
