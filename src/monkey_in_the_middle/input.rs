use std::path::Path;

use crate::prelude::{Error, Result};
use crate::utils::read_lines;

use super::Monkey;

pub fn read_values(file_path: &Path) -> Result<Vec<Monkey>> {
    let lines: Vec<String> = read_lines(file_path)?.into_iter().flatten().collect();

    let mut monkeys = vec![];

    for i in lines.split(|l| l.is_empty()) {
        let m = create_monkey(i)?;
        monkeys.push(m)
    }

    monkeys.sort_by_key(|m| m.index);

    Ok(monkeys)
}

fn create_monkey(lines: &[String]) -> Result<Monkey> {
    if lines.len() == 6 {
        let index = parse_monkey_index(&lines[0])?;
        let items = parse_items(&lines[1])?;
        let op = parse_op(&lines[2])?;
        let div = number_at_end_of_line(&lines[3])?;
        let ti = number_at_end_of_line(&lines[4])? as usize;
        let fi = number_at_end_of_line(&lines[5])? as usize;

        Ok(Monkey {
            index,
            inspections: 0,
            items,
            op,
            div,
            ti,
            fi,
        })
    } else {
        Err(Error::Generic(format!("lines.len() != 6 for {lines:?}")))
    }
}

fn parse_monkey_index(s: &str) -> Result<usize> {
    let (_, i) = s
        .split_once(' ')
        .ok_or(Error::Generic(format!("{s} not splittable by whitespace")))?;

    let i: usize = i.trim_matches(':').parse()?;
    Ok(i)
}

fn parse_items(s: &str) -> Result<Vec<u64>> {
    let (_, items) = s
        .split_once(':')
        .ok_or(Error::Generic(format!("{s} not splittable by :")))?;

    items
        .trim()
        .split(',')
        .into_iter()
        .map(|i| i.trim().parse::<u64>().map_err(Error::ParseInt))
        .collect()
}

fn parse_op(s: &str) -> Result<Box<dyn Fn(u64) -> u64>> {
    let (_, rhs) = s
        .split_once('=')
        .ok_or(Error::Generic(format!("{s} not splittable by =")))?;

    let v: Vec<&str> = rhs.split_whitespace().take(3).collect();

    if v.len() != 3 {
        return Err(Error::Generic(format!("rhs size not 3: {rhs}")));
    }

    let rop = match v[2] {
        "old" => RightOp::Old,
        n => RightOp::Val(n.parse()?),
    };

    let op: Box<dyn Fn(u64) -> u64> = match v[1] {
        "+" => match rop {
            RightOp::Old => Box::new(move |x| add2(x, x)()),
            RightOp::Val(n) => Box::new(move |x| add2(x, n)()),
        },
        "*" => match rop {
            RightOp::Old => Box::new(move |x| mul2(x, x)()),
            RightOp::Val(n) => Box::new(move |x| mul2(x, n)()),
        },
        o => return Err(Error::Generic(format!("unknown operator {o}"))),
    };

    Ok(op)
}

fn number_at_end_of_line(s: &str) -> Result<u64> {
    let s = s
        .split_whitespace()
        .last()
        .ok_or(Error::Generic(format!("No Last in {s}")))?;

    s.parse().map_err(Error::ParseInt)
}

enum RightOp {
    Old,
    Val(u64),
}

fn add2(a: u64, b: u64) -> Box<dyn Fn() -> u64> {
    Box::new(move || a + b)
}

fn mul2(a: u64, b: u64) -> Box<dyn Fn() -> u64> {
    Box::new(move || a * b)
}
