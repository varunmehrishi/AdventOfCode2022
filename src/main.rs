use std::{path::Path, env};
#[allow(unused)]

mod utils;
mod error;
mod prelude;
mod calorie_counting;
mod rock_paper_scissors;
mod rucksack_reorganization;
mod camp_cleanup;
mod supply_stacks;
mod tuning_trouble;
mod no_space_left_on_device;
mod treetop_tree_house;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage ./aoc path_to_directory_containing_input_files");
        return;
    }

    let base = Path::new(&args[1]);

    calorie_counting::solve(&base.join("calorie_counting.txt"));
    rock_paper_scissors::solve(&base.join("rock_paper_scissors.txt"));
    rucksack_reorganization::solve(&base.join("rucksack_reorganization.txt"));
    camp_cleanup::solve(&base.join("camp_cleanup.txt"));
    supply_stacks::solve(&base.join("supply_stacks.txt"));
    tuning_trouble::solve(&base.join("tuning_trouble.txt"));
    no_space_left_on_device::solve(&base.join("no_space_left_on_device.txt"));
    treetop_tree_house::solve(&base.join("treetop_tree_house.txt"));
}
