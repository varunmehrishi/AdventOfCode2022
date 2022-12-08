use std::path::Path;

use crate::prelude::Result;
use crate::utils::read_lines;

pub fn read_values(file_path: &Path) -> Result<Vec<Vec<u8>>> {
    let lines = read_lines(file_path)?;

    let mut values = vec![];
    for line in lines {
        let ip = line?;
        let mut bytes = ip.into_bytes();
        for b in &mut bytes {
            *b -= b'0';
        }
        values.push(bytes);
    }

    Ok(values)
}
