use std::path::Path;

mod input;
mod packet;

use packet::Packet;
use serde_json::from_str;

pub fn solve(path: &Path) {
    let values = input::read_values(path)
        .unwrap_or_else(|e| panic!("Failed to read input file for {path:?} due to {e}"));

    let index_sum: usize = values
        .iter()
        .enumerate()
        .filter_map(|(i, (a, b))| if a <= b { Some(i + 1) } else { None })
        .sum();

    println!("Distress Signal Part 1: {index_sum}");

    let mut packets: Vec<Packet> = values.into_iter().flat_map(|(a, b)| vec![a, b]).collect();

    let p2: Packet = from_str("[[2]]").expect("Parsing p2 failed");
    let p6: Packet = from_str("[[6]]").expect("Parsing p6 failed");
    packets.extend(vec![p2.clone(), p6.clone()]);
    packets.sort();

    let i2 = packets
        .iter()
        .position(|p| *p == p2)
        .expect("Did not find p2")
        + 1;
    let i6 = packets
        .iter()
        .position(|p| *p == p6)
        .expect("Did not find p6")
        + 1;

    println!("Distress Signal Part 2: {}", i2 * i6);
}
