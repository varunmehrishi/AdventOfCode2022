use std::path::Path;

use serde_json::from_str;

use crate::prelude::Result;
use crate::utils::read_lines;

use super::Packet;

pub fn read_values(file_path: &Path) -> Result<Vec<(Packet, Packet)>> {
    let lines: Vec<String> = read_lines(file_path)?.into_iter().flatten().collect();

    lines
        .split(|l| l.is_empty())
        .map(|v| Ok((from_str(&v[0])?, from_str(&v[1])?)))
        .collect()
}
