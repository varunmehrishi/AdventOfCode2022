use std::{io::Error, path::Path};

use crate::utils::read_lines;

pub fn read_values(file_path: &Path) -> Result<Vec<Vec<u32>>, Error> {
    let mut values = vec![];
    let lines = read_lines(file_path)?;

    let mut cur = vec![];
    for line in lines {
        let ip = line?;
        // values.push(ip);
        if ip.is_empty() {
            values.push(cur.clone());
            cur.clear();
        } else {
            let cal: u32 = ip.parse().expect("Unexpected Value");
            cur.push(cal);
        }
    }

    Ok(values)
}
