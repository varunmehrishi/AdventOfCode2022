use std::{
    collections::{BTreeSet, HashMap, HashSet},
    path::Path,
};

mod input;

type SKey = (usize, i64, BTreeSet<(i64, i64)>);

pub fn solve(path: &Path) {
    let dirs = input::read_values(path)
        .unwrap_or_else(|e| panic!("Failed to read input file for {path:?} due to {e}"));

    let mut d = 0;
    let mut top_heights = [0; 7];
    let mut rock_points = HashSet::from_iter((1..=7).map(|y| (0, y)));

    let mut cache: HashMap<SKey, (i64, i64)> = HashMap::new();

    let mut i = 0;
    let mut added = 0;
    while i < 1000000000000 {
        let mx_h: i64 = *top_heights.iter().max().unwrap();

        if i == 2023 {
            println!("Pyroclastic Flow Part 1 {}", added + mx_h);
        }

        let mut r = get_rock(mx_h + 4, 3, i);

        loop {
            let next_dir = dirs[d];
            d = (d + 1) % dirs.len();

            if !intersects_wall(r.move_rock(next_dir))
                && !intersects_rocks(r.move_rock(next_dir), &rock_points)
            {
                r = r.move_rock(next_dir);
            }

            if intersects_rocks(r.move_rock(Dir::Down), &rock_points) {
                let points = r.get_points();
                points.iter().for_each(|(x, y)| {
                    top_heights[(y - 1) as usize] = top_heights[(y - 1) as usize].max(*x);
                });
                rock_points.extend(points);

                let mx_h: i64 = *top_heights.iter().max().unwrap();
                let key = (d, i % 5, lastrows(&rock_points));

                if i > 2022 {
                    if let Some((oi, oh)) = cache.get(&key) {
                        let di = i - oi;
                        let jumps = (1000000000000 - i) / di;
                        let dh = mx_h - oh;
                        added += jumps * dh;
                        i += jumps * di;
                    } else {
                        cache.insert(key, (i, mx_h));
                    }
                }

                break;
            } else {
                r = r.move_rock(Dir::Down);
            }
        }
        i += 1;
    }

    // let mut grid = vec![vec![' '; 8]; 20];
    //
    // rock_points
    //     .into_iter()
    //     .for_each(|(x, y)| grid[x as usize][y as usize] = '#');
    //
    // grid.into_iter()
    //     .rev()
    //     .for_each(|v| println!("{}", v.iter().collect::<String>()));
    //
    println!(
        "Pyroclastic Flow Part 2: {}",
        added + top_heights.iter().max().unwrap()
    );
}

fn intersects_rocks(rock: Rock, points: &HashSet<(i64, i64)>) -> bool {
    points.intersection(&rock.get_points()).next().is_some()
}

fn lastrows(points: &HashSet<(i64, i64)>) -> BTreeSet<(i64, i64)> {
    let mx_h = points.iter().map(|(x, _)| x).max().unwrap();

    points
        .iter()
        .filter(|(x, _)| mx_h - x <= 5)
        .map(|(x, y)| (mx_h - x, *y))
        .collect()
}

fn intersects_wall(rock: Rock) -> bool {
    rock.get_points().into_iter().any(|(_, y)| y == 0 || y == 8)
}

#[derive(Copy, Clone, Debug)]
enum Rock {
    Dash { x: i64, y: i64 },
    Plus { x: i64, y: i64 },
    L { x: i64, y: i64 },
    Line { x: i64, y: i64 },
    Box { x: i64, y: i64 },
}

impl Rock {
    fn get_points(self) -> HashSet<(i64, i64)> {
        HashSet::from_iter(match self {
            Rock::Dash { x, y } => vec![(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],
            Rock::Plus { x, y } => vec![
                (x, y + 1),
                (x + 1, y),
                (x + 1, y + 1),
                (x + 1, y + 2),
                (x + 2, y + 1),
            ],
            Rock::L { x, y } => vec![
                (x, y),
                (x, y + 1),
                (x, y + 2),
                (x + 1, y + 2),
                (x + 2, y + 2),
            ],
            Rock::Line { x, y } => vec![(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],
            Rock::Box { x, y } => vec![(x, y), (x, y + 1), (x + 1, y), (x + 1, y + 1)],
        })
    }

    fn move_rock(&self, dir: Dir) -> Self {
        let (dx, dy) = dir.get_differentials();

        match self {
            Rock::Dash { x, y } => Rock::Dash {
                x: x + dx,
                y: y + dy,
            },
            Rock::Plus { x, y } => Rock::Plus {
                x: x + dx,
                y: y + dy,
            },
            Rock::L { x, y } => Rock::L {
                x: x + dx,
                y: y + dy,
            },
            Rock::Line { x, y } => Rock::Line {
                x: x + dx,
                y: y + dy,
            },
            Rock::Box { x, y } => Rock::Box {
                x: x + dx,
                y: y + dy,
            },
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Dir {
    Left,
    Right,
    Down,
}

impl Dir {
    fn get_differentials(self) -> (i64, i64) {
        match self {
            Dir::Left => (0, -1),
            Dir::Right => (0, 1),
            Dir::Down => (-1, 0),
        }
    }
}

fn get_rock(x: i64, y: i64, i: i64) -> Rock {
    match i % 5 {
        0 => Rock::Dash { x, y },
        1 => Rock::Plus { x, y },
        2 => Rock::L { x, y },
        3 => Rock::Line { x, y },
        _ => Rock::Box { x, y },
    }
}
