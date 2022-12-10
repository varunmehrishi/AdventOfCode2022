use std::path::Path;

use crate::prelude::{Error, Result};
use crate::utils::read_lines;

use super::Op;

pub fn read_values(file_path: &Path) -> Result<Vec<Op>> {
    let lines = read_lines(file_path)?;

    let mut values = vec![];
    for line in lines {
        let ip = line?;

        let mut it = ip.split_whitespace();
        values.push(match (it.next(), it.next()) {
            (Some("noop"), None) => Op::Noop,
            (Some("addx"), Some(n)) => {
                Op::Addx(n.parse()?)
            }
            _ => return Err(Error::Generic(format!("Unkown line in input {ip}"))),
        })
    }

    Ok(values)
}
