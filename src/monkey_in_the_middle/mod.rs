use std::path::Path;

mod input;
mod monkey;

use monkey::Monkey;

pub fn solve(path: &Path) {
    let mut monkeys = input::read_values(path)
        .unwrap_or_else(|e| panic!("Failed to read input file for {path:?} due to {e}"));

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let m = &mut monkeys[i];
            m.inspect_items();
            m.decrease_worry();
            let (t, f) = m.test();
            let (ti, fi) = (m.ti, m.fi);
            monkeys[ti].items.extend(t);
            monkeys[fi].items.extend(f);
        }
    }

    let mut v: Vec<_> = monkeys.iter().map(|m| m.inspections).collect();
    v.sort_unstable();

    println!(
        "Monkey in the middle part 1: {}",
        v.iter().rev().take(2).product::<u64>()
    );

    let mut monkeys = input::read_values(path)
        .unwrap_or_else(|e| panic!("Failed to read input file for {path:?} due to {e}"));

    let div: u64 = monkeys.iter().map(|m| m.div).product();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let m = &mut monkeys[i];
            m.inspect_items_2(div);
            let (t, f) = m.test();
            let (ti, fi) = (m.ti, m.fi);
            monkeys[ti].items.extend(t);
            monkeys[fi].items.extend(f);
        }
    }

    let mut v: Vec<_> = monkeys.iter().map(|m| m.inspections).collect();
    v.sort_unstable();

    println!(
        "Monkey in the middle part 2: {}",
        v.iter().rev().take(2).product::<u64>()
    );
}
