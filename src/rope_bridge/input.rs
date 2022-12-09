use std::path::Path;

use crate::prelude::{Error, Result};
use crate::utils::read_lines;

use super::Direction;

pub fn read_values(file_path: &Path) -> Result<Vec<Direction>> {
    let lines = read_lines(file_path)?;

    let mut values = vec![];
    for line in lines {
        let ip = line?;

        let (c, n) = ip.split_once(' ').ok_or(Error::Generic(format!(
            "Unable to split once on whitespace: \"{ip}\""
        )))?;

        let n: u32 = n.parse()?;

        for _ in 0..n {
            values.push(match c {
                "L" => Direction::Left,
                "R" => Direction::Right,
                "U" => Direction::Up,
                "D" => Direction::Down,
                c => return Err(Error::Generic(format!("Unkown Direction {c}"))),
            })
        }
    }

    Ok(values)
}
