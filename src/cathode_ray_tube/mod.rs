use std::path::Path;

mod input;

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Noop,
    Addx(i32),
}

pub fn solve(path: &Path) {
    let operations = input::read_values(path)
        .unwrap_or_else(|e| panic!("Failed to read input file for {path:?} due to {e}"));

    let mut cycle = 0;
    let mut x = 1;

    let mut signal_strengths = 0;
    let mut pixels = Vec::with_capacity(240);

    for op in operations {
        let c = cycle_count_for_op(op);
        for i in 1..=c {
            cycle += 1;
            if [20, 60, 100, 140, 180, 220].contains(&cycle) {
                signal_strengths += cycle * x;
            }

            let pos = (cycle - 1) % 40;

            if [x - 1, x, x + 1].contains(&pos) {
                pixels.push('#');
            } else {
                pixels.push(' ');
            }

            if i == c {
                match op {
                    Op::Noop => {}
                    Op::Addx(n) => x += n,
                }
            }
        }
    }

    println!("Cathode Ray Part 1 {signal_strengths}");
    println!("Cathode Ray Part 2");
    pixels.chunks(40).for_each(|c| {
        println!("{}", c.iter().collect::<String>());
    })
}

fn cycle_count_for_op(o: Op) -> i32 {
    match o {
        Op::Noop => 1,
        Op::Addx(_) => 2,
    }
}
