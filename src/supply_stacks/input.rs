use std::collections::HashMap;
use std::path::Path;

use crate::prelude::{Error, Result};
use crate::utils::read_lines;

use super::MoveOp;

type Stack = Vec<char>;

pub fn read_values(file_path: &Path) -> Result<(Vec<Stack>, Vec<MoveOp>)> {
    let mut values = vec![];
    let lines = read_lines(file_path)?;
    let mut stacks = None;

    for line in lines {
        let ip = line?;
        if ip.is_empty() {
            stacks = create_stacks(values);
            values = vec![];
            continue;
        }
        values.push(ip);
    }

    let ops: Option<Vec<MoveOp>> = values.into_iter().map(create_moveop).collect();

    Ok((
        stacks.ok_or_else(|| Error::Generic("Could not create stacks".to_owned()))?,
        ops.ok_or_else(|| Error::Generic("Could not create Move operations".to_owned()))?,
    ))
}

fn create_stacks(mut stack_strings: Vec<String>) -> Option<Vec<Stack>> {
    let index_line = stack_strings.pop()?.chars().collect::<Vec<_>>();
    let index_map = get_index_mapping(index_line)?;

    let mut stacks = vec![vec![]; index_map.len()];

    for layer in stack_strings.iter().rev() {
        let chars = layer.chars().collect::<Vec<_>>();
        for (&k, &v) in &index_map {
            match chars[v] {
                ' ' => {}
                c => {
                    stacks[k].push(c);
                }
            }
        }
    }

    Some(stacks)
}

fn get_index_mapping(s: Vec<char>) -> Option<HashMap<usize, usize>> {
    let mut index_map = HashMap::new();
    for (i, c) in s.iter().enumerate() {
        if let '1'..='9' = c {
            let v: usize = (c.to_digit(10)? - 1) as _;
            index_map.insert(v, i);
        }
    }
    Some(index_map)
}

fn create_moveop(s: String) -> Option<MoveOp> {
    let values: Vec<&str> = s.split_whitespace().into_iter().collect();

    if values.len() == 6 {
        let count = values[1].parse().ok()?;
        let from: usize = values[3].parse().ok()?;
        let to: usize = values[5].parse().ok()?;
        Some(MoveOp { count, from: from - 1, to: to - 1})
    } else {
        println!("{s}");
        None
    }
}
