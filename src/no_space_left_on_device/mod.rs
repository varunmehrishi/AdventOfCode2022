use std::{cell::RefCell, path::Path, rc::Rc};

mod input;

#[derive(Debug)]
pub struct DirS {
    pub name: String,
    pub items: Vec<Item>,
}

impl DirS {
    pub fn new(name: String) -> Self {
        Self {
            name,
            items: vec![],
        }
    }
}

#[derive(Clone, Debug)]
pub enum Item {
    File(String, u64),
    Dir(Rc<RefCell<DirS>>),
}

pub fn solve(path: &Path) {
    let root_dir = input::read_values(path)
        .unwrap_or_else(|e| panic!("Failed to read input file for {path:?} due to {e}"));

    let item = Item::Dir(root_dir);
    let mut v = vec![];
    let root_size = find_directories(item, &mut v);

    let ans: u64 = v
        .iter()
        .map(|(_, sz)| *sz)
        .filter(|sz| *sz <= 100_000u64)
        .sum();

    println!("No Space Left On Device Part 1: {ans}");

    let free_space_available = 70_000_000u64 - root_size;

    if let Some(min_size) = v
        .iter()
        .map(|(_, sz)| *sz)
        .filter(|sz| *sz + free_space_available >= 30_000_000)
        .min()
    {
        println!("No Space Left On Device Part 2: {min_size}");
    } else {
        println!("No Space Left On Device Part 2 - not found");
    }
}

fn find_directories(item: Item, result: &mut Vec<(Rc<RefCell<DirS>>, u64)>) -> u64 {
    match item {
        Item::File(_, n) => n,
        Item::Dir(dirs) => {
            let sz = dirs
                .borrow()
                .items
                .iter()
                .map(|it| find_directories(it.clone(), result))
                .sum();

            result.push((dirs, sz));

            sz
        }
    }
}
