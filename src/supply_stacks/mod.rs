use std::path::Path;

mod input;

pub struct MoveOp {
    from: usize,
    to: usize,
    count: u32,
}

pub fn solve(path: &Path) {
    let (mut stacks, ops) = input::read_values(path)
        .unwrap_or_else(|e| panic!("Failed to read input file for {path:?} due to {e}"));
    for MoveOp { from, to, count } in ops {
        for _ in 0..count {
            if let Some(c) = stacks[from].pop() {
                stacks[to].push(c)
            } else {
                println!("Found Invalid Move - Empty Stack");
            }
        }
    }
    print!("Supply Stacks Part 1 ");
    stacks
        .iter()
        .map(|s| s.last().unwrap_or(&' '))
        .chain(Some(&'\n'))
        .for_each(|c| print!("{c}"));

    let mut aux = vec![];
    let (mut stacks, ops) = input::read_values(path)
        .unwrap_or_else(|e| panic!("Failed to read input file for {path:?} due to {e}"));
    for MoveOp { from, to, count } in ops {
        let n = stacks[from].len();
        aux.extend(stacks[from].drain(n - count as usize..));
        stacks[to].extend(aux.drain(0..));
    }
    print!("Supply Stacks Part 2 ");
    stacks
        .iter()
        .map(|s| s.last().unwrap_or(&' '))
        .chain(Some(&'\n'))
        .for_each(|c| print!("{c}"));
}
