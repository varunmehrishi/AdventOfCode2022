use std::path::Path;

use crate::prelude::Error::IO;
use crate::prelude::Result;
use crate::utils::read_lines;

use super::Dir;

pub fn read_values(file_path: &Path) -> Result<Vec<Dir>> {
    let lines: Result<Vec<String>> = read_lines(file_path)?
        .into_iter()
        .map(|r| r.map_err(IO))
        .collect();


    Ok(lines?[0].trim().chars().map(|c| match c {
        '<' => Dir::Left,
        '>' => Dir::Right,
        _ => unreachable!(),
    }).collect())

}
