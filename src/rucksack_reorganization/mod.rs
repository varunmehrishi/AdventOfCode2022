use std::{collections::HashSet, path::Path};

mod input;

pub fn solve(path: &Path) {
    let rucksacks =
        input::read_values(path).unwrap_or_else(|_| panic!("Unable to read {path:?}"));

    let mut score = 0;
    for rucksack in &rucksacks {
        let n = rucksack.len();

        let (h1, h2) = rucksack.split_at(n / 2);

        let sum: i32 = get_common_chars(h1, h2)
            .into_iter()
            .map(compute_priority)
            .sum();
        score += sum;
    }
    println!("Rucksack Part 1 {score}");

    let mut score = 0;
    rucksacks.chunks_exact(3).for_each(|chunk| {
        let a = &chunk[0];
        let b = &chunk[1];
        let c = &chunk[2];

        let ab: Vec<_> = get_common_chars(a, b).into_iter().collect();
        let common = get_common_chars(&ab, c);

        let sum: i32 = common.into_iter().map(compute_priority).sum();
        score += sum;
    });
    println!("Rucksack Part 2 {score}");
}

fn get_common_chars(a: &[char], b: &[char]) -> HashSet<char> {
    let a: HashSet<_> = a.iter().cloned().collect();
    b.iter().cloned().filter(|c| a.contains(c)).collect()
}

fn compute_priority(c: char) -> i32 {
    match c {
        'a'..='z' => ((c as u8) - b'a' + 1) as i32,
        'A'..='Z' => ((c as u8) - b'A' + 27) as i32,
        _ => unreachable!("Unexpected Character"),
    }
}
