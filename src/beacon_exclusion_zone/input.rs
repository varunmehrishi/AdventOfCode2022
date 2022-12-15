use std::path::Path;

use crate::prelude::Error::{Generic, IO};
use crate::prelude::Result;
use crate::utils::read_lines;

type Point = (i32, i32);

pub fn read_values(file_path: &Path) -> Result<Vec<(Point, Point)>> {
    let lines: Result<Vec<String>> = read_lines(file_path)?
        .into_iter()
        .map(|r| r.map_err(IO))
        .collect();

    lines?
        .into_iter()
        .map(|l| {
            l.split_once(": ")
                .ok_or(Generic(format!("No : in {l}")))
                .and_then(|(a, b)| Ok((get_coordinates(a)?, get_coordinates(b)?)))
        })
        .collect()
}

fn get_coordinates(s: &str) -> Result<(i32, i32)> {
    let mut it = s.split_whitespace().rev();
    let pat = [',', ':', 'x', 'y', '='];
    let y: i32 = it
        .next()
        .ok_or(Generic(format!("No Y in {s}")))?
        .trim_matches(|c: char| pat.contains(&c))
        .parse()?;

    let x: i32 = it
        .next()
        .ok_or(Generic(format!("No X in {s}")))?
        .trim_matches(|c: char| pat.contains(&c))
        .parse()?;

    Ok((x, y))
}
