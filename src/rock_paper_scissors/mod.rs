use std::path::Path;

mod input;

#[derive(Debug, Clone, Copy)]
pub enum Hand {
    Rock,
    Paper,
    Scissor
}

#[derive(Debug, Clone, Copy)]
pub enum Outcome {
    Win,
    Lose,
    Draw
}

fn outcome(op: Hand, my: Hand) -> Outcome {
    match (op, my) {
        (Hand::Rock, Hand::Rock) => Outcome::Draw,
        (Hand::Rock, Hand::Paper) => Outcome::Win,
        (Hand::Rock, Hand::Scissor) => Outcome::Lose,
        (Hand::Paper, Hand::Rock) => Outcome::Lose,
        (Hand::Paper, Hand::Paper) => Outcome::Draw,
        (Hand::Paper, Hand::Scissor) => Outcome::Win,
        (Hand::Scissor, Hand::Rock) => Outcome::Win,
        (Hand::Scissor, Hand::Paper) => Outcome::Lose,
        (Hand::Scissor, Hand::Scissor) => Outcome::Draw,
    }
}

fn score_for_hand(hand: Hand) -> i32 {
    match hand {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissor => 3,
    }
}

fn score_for_outcome(outcome: Outcome) -> i32 {
    match outcome {
        Outcome::Win => 6,
        Outcome::Lose => 0,
        Outcome::Draw => 3,
    }
}

fn get_play(op: Hand, outcome: Outcome) -> Hand {
    match (op, outcome) {
        (Hand::Rock, Outcome::Win) => Hand::Paper,
        (Hand::Rock, Outcome::Lose) => Hand::Scissor,
        (Hand::Rock, Outcome::Draw) => Hand::Rock,
        (Hand::Paper, Outcome::Win) => Hand::Scissor,
        (Hand::Paper, Outcome::Lose) => Hand::Rock,
        (Hand::Paper, Outcome::Draw) => Hand::Paper,
        (Hand::Scissor, Outcome::Win) => Hand::Rock,
        (Hand::Scissor, Outcome::Lose) => Hand::Paper,
        (Hand::Scissor, Outcome::Draw) => Hand::Scissor,
    }
}

pub fn solve(path: &Path) {
    let values = input::read_values(path).unwrap_or_else(|_| panic!("Unable to read {path:?}"));
    let mut score = 0;
    for (op, my) in values {
        let outcome = outcome(op, my);
        score += score_for_hand(my) + score_for_outcome(outcome);
    }
    println!("Score for Part 1 {score}");

    let values = input::read_values_part_2(path).unwrap_or_else(|_| panic!("Unable to read {path:?}"));
    let mut score = 0;
    for (op, outcome) in values {
        let my = get_play(op, outcome);
        score += score_for_hand(my) + score_for_outcome(outcome);
    }
    println!("Score for Part 2 {score}");
}
