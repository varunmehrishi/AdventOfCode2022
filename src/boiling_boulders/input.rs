use std::path::Path;

use crate::prelude::Error::IO;
use crate::prelude::Result;
use crate::utils::read_lines;

use super::Cube;

pub fn read_values(file_path: &Path) -> Result<Vec<Cube>> {
    let lines: Result<Vec<String>> = read_lines(file_path)?
        .into_iter()
        .map(|r| r.map_err(IO))
        .collect();

    lines?
        .into_iter()
        .map(Cube::try_from)
        .collect()
}
