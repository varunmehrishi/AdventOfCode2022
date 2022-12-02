use std::{io::Error, path::Path};

use crate::utils::read_lines;

use super::{Hand, Outcome};

pub fn read_values(file_path: &Path) -> Result<Vec<(Hand, Hand)>, Error> {
    let mut values = vec![];
    let lines = read_lines(file_path)?;

    for line in lines {
        let ip = line?;

        if let Some((opponent_hand, my_hand)) = ip.split_once(' ') {
            let op = match opponent_hand {
                "A" => Hand::Rock,
                "B" => Hand::Paper,
                "C" => Hand::Scissor,
                _ => unreachable!("Unknown Play")
            };
            

            let my = match my_hand {
                "X" => Hand::Rock,
                "Y" => Hand::Paper,
                "Z" => Hand::Scissor,
                _ => unreachable!("Unknown Play")
            };

            values.push((op, my));
        } else {
            panic!("Unexpected terminators");
        }
    }

    Ok(values)
}

pub fn read_values_part_2(file_path: &Path) -> Result<Vec<(Hand, Outcome)>, Error> {
    let mut values = vec![];
    let lines = read_lines(file_path)?;

    for line in lines {
        let ip = line?;

        if let Some((opponent_hand, outcome)) = ip.split_once(' ') {
            let op = match opponent_hand {
                "A" => Hand::Rock,
                "B" => Hand::Paper,
                "C" => Hand::Scissor,
                _ => unreachable!("Unknown Play")
            };
            

            let out = match outcome {
                "X" => Outcome::Lose,
                "Y" => Outcome::Draw,
                "Z" => Outcome::Win,
                _ => unreachable!("Unknown Outcome")
            };

            values.push((op, out));
        } else {
            panic!("Unexpected terminators");
        }
    }

    Ok(values)
}
