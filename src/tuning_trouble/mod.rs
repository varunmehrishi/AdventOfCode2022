use std::path::Path;

mod input;

pub fn solve(path: &Path) {
    let s = input::read_values(path)
        .unwrap_or_else(|e| panic!("Failed to read input file for {path:?} due to {e}"));

    let chars: Vec<_> = s.chars().collect();

    for (p, ws) in [4usize, 14].into_iter().enumerate() {
        chars
            .windows(ws)
            .enumerate()
            .filter_map(|(i, w)| {
                let mut v: Vec<_> = w.to_vec();
                v.sort_unstable();
                v.dedup();

                if v.len() == ws {
                    Some(i + ws)
                } else {
                    None
                }
            })
            .take(1)
            .for_each(|v| println!("Tuning Trouble Part {}: {}", p + 1, v));
    }
}
