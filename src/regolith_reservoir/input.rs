use std::path::Path;

use crate::prelude::Error::{Generic, IO};
use crate::prelude::Result;
use crate::utils::read_lines;

pub fn read_values(file_path: &Path) -> Result<Vec<Vec<(i32, i32)>>> {
    let lines: Result<Vec<String>> = read_lines(file_path)?
        .into_iter()
        .map(|r| r.map_err(IO))
        .collect();

    lines?
        .into_iter()
        .map(|l| {
            l.split("->")
                .map(|s| {
                    if let Some((x, y)) = s.trim().split_once(',') {
                        Ok((x.parse()?, y.parse()?))
                    } else {
                        Err(Generic(format!("{s:?}")))
                    }
                })
                .collect::<Result<Vec<(i32, i32)>>>()
        })
        .collect()
}
