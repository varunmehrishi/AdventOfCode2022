use std::path::Path;

use crate::prelude::Result;
use crate::utils::read_lines;

pub fn read_values(file_path: &Path) -> Result<Vec<Vec<char>>> {
    let lines = read_lines(file_path)?
        .into_iter()
        .flatten()
        .map(|l| l.chars().collect())
        .collect();

    Ok(lines)
}
