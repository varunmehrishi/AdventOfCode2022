use serde::{Deserialize, Serialize};
use std::cmp::Ordering::{Greater, Less};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum Packet {
    Num(i32),
    Sub(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Packet::Num(a), Packet::Num(b)) => a.partial_cmp(b),
            (Packet::Num(a), Packet::Sub(vals)) => match vals.as_slice() {
                [] => Some(Greater),
                [b, ..] => Packet::Num(*a).partial_cmp(b),
            },
            (Packet::Sub(vals), Packet::Num(b)) => match vals.as_slice() {
                [] => Some(Less),
                [a, ..] => a.partial_cmp(&Packet::Num(*b)),
            },
            (Packet::Sub(vals_a), Packet::Sub(vals_b)) => vals_a.partial_cmp(vals_b),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Num(a), Packet::Num(b)) => a.cmp(b),
            (Packet::Num(a), Packet::Sub(vals)) => match vals.as_slice() {
                [] => Greater,
                [b, ..] => Packet::Num(*a).cmp(b),
            },
            (Packet::Sub(vals), Packet::Num(b)) => match vals.as_slice() {
                [] => Less,
                [a, ..] => a.cmp(&Packet::Num(*b)),
            },
            (Packet::Sub(vals_a), Packet::Sub(vals_b)) => vals_a.cmp(vals_b),
        }
    }
}
