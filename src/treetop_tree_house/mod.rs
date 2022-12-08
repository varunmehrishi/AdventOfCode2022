use std::{cell::Cell, cmp::Ordering, path::Path};

mod input;

pub fn solve(path: &Path) {
    let matrix = input::read_values(path)
        .unwrap_or_else(|e| panic!("Failed to read input file for {path:?} due to {e}"));

    let directions = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    let m = matrix.len();
    let n = matrix[0].len();
    let mut visible = 0;

    for i in 0..m {
        for j in 0..n {
            if is_boundary((i, j), (m, n)) {
                visible += 1;
                continue;
            }

            let val = matrix[i][j];
            for direction in directions {
                if get_values_till_boundry((i, j), &matrix, direction)
                    .into_iter()
                    .all(|v| v < val)
                {
                    visible += 1;
                    break;
                }
            }
        }
    }

    println!("Treetop Tree House Part 1: {visible}");

    let mut max_scenic_score = 0;

    for i in 0..m {
        for j in 0..n {
            if is_boundary((i, j), (m, n)) {
                continue;
            }

            let val = matrix[i][j];
            let mut score = 1;
            for direction in directions {
                let eq_gt_found = Cell::new(false);

                score *= get_values_till_boundry((i, j), &matrix, direction)
                    .into_iter()
                    .take_while(|v| {
                        if eq_gt_found.get() {
                            false
                        } else {
                            match v.cmp(&val) {
                                Ordering::Less => true,
                                _ => {
                                    eq_gt_found.set(true);
                                    true
                                }
                            }
                        }
                    })
                    .count();
            }

            if score > max_scenic_score {
                max_scenic_score = score;
            }
        }
    }
    println!("Treetop Tree House Part 2: {max_scenic_score}");
}

pub fn is_boundary((i, j): (usize, usize), (m, n): (usize, usize)) -> bool {
    i == 0 || j == 0 || i == m || j == n
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_values_till_boundry(
    (i, j): (usize, usize),
    matrix: &[Vec<u8>],
    direction: Direction,
) -> Vec<u8> {
    let m = matrix.len() as i32;
    let n = matrix[0].len() as i32;

    let (mut i, mut j) = (i as i32, j as i32);

    let (di, dj) = match direction {
        Direction::Up => (-1, 0),
        Direction::Down => (1, 0),
        Direction::Left => (0, -1),
        Direction::Right => (0, 1),
    };

    i += di;
    j += dj;

    let mut values = vec![];

    while 0 <= i && i < m && 0 <= j && j < n {
        values.push(matrix[i as usize][j as usize]);
        i += di;
        j += dj;
    }

    values
}
