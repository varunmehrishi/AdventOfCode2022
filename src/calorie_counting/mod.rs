use std::path::Path;

mod input;

pub fn solve(path: &Path) {
   let values = input::read_values(path).unwrap_or_else(|_| panic!("Unable to read file {:?}", path));

   let mut calories: Vec<_>= values.iter().map(|v| v.iter().sum::<u32>()).collect();

   calories.sort_unstable();

   println!("Calorie Counting Part 1 {:?}", calories.last().unwrap());
   println!("Calorie Counting Part 2 {:?}", calories.iter().rev().take(3).sum::<u32>());
}
