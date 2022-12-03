use std::{io::Error, path::Path};

use crate::utils::read_lines;

pub fn read_values(file_path: &Path) -> Result<Vec<Vec<char>>, Error> {
    let mut values = vec![];
    let lines = read_lines(file_path)?;

    for line in lines {
        let ip = line?;
        // values.push(ip);
        values.push(ip.chars().into_iter().collect());
    }

    Ok(values)
}
