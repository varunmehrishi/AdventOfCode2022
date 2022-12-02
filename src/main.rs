use std::{path::Path, env};

mod utils;
mod calorie_counting;
mod rock_paper_scissors;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage ./aoc path_to_directory_containing_input_files");
        return;
    }

    let base = Path::new(&args[1]);

    calorie_counting::solve(&base.join("calorie_counting.txt"));
    rock_paper_scissors::solve(&base.join("rock_paper_scissors.txt"));
}
