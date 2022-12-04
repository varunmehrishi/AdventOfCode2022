use std::path::Path;

use crate::utils::read_lines;
use crate::prelude::{Result, Error};

use super::Interval;

pub fn read_values(file_path: &Path) -> Result<Vec<(Interval, Interval)>> {
    let mut values = vec![];
    let lines = read_lines(file_path)?;

    for line in lines {
        let ip = line?;
        // values.push(ip);
        if let Some((a, b)) = ip.split_once(',') {
            let a = create_interval(a)?;
            let b = create_interval(b)?;
            values.push((a, b))
        } else {
            return Err(Error::Generic(format!("No , in {ip}")))
        }
    }

    Ok(values)
}

pub fn create_interval(s: &str) -> Result<Interval> {
    if let Some((start, stop)) = s.split_once('-') {
        let start: u32 = start.parse()?;
        let stop: u32 = stop.parse()?;

        Ok(Interval::new(start, stop))
    } else {
        Err(Error::Generic(format!("No - in {s}")))
    }
}
