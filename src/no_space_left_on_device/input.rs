use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use crate::prelude::{Error, Result};
use crate::utils::read_lines;

use super::{DirS, Item};

pub fn read_values(file_path: &Path) -> Result<Rc<RefCell<DirS>>> {
    let lines = read_lines(file_path)?;

    let root = Rc::new(RefCell::new(DirS::new("/".to_owned())));
    let mut stack = vec![root.clone()];

    for line in lines {
        let ip = line?;
        let li = parse(ip)?;
        // println!("{li:?}");
        match li {
            LineItem::ChangeDir(dir_name) => match dir_name.as_str() {
                "/" => {}
                ".." => {
                    stack.pop();
                }
                name => {
                    let mut match_dir = None;
                    if let Some(last) = stack.last() {
                        let dir = last.borrow();
                        for item in &dir.items {
                            match item {
                                Item::File(_, _) => {}
                                Item::Dir(dir) => {
                                    if dir.borrow().name == name {
                                        match_dir = Some(dir.clone());
                                    }
                                }
                            }
                        }
                    } else {
                        return Err(Error::Generic("Empty Stack".to_owned()));
                    }
                    stack.push(match_dir.expect("Did not find cd dirname match"));
                }
            },
            LineItem::List => {}
            LineItem::Dir(dir_name) => {
                if let Some(last) = stack.last() {
                    let mut dir = last.borrow_mut();
                    dir.items
                        .push(Item::Dir(Rc::new(RefCell::new(DirS::new(dir_name)))));
                } else {
                    return Err(Error::Generic("Empty Stack".to_owned()));
                }
            }
            LineItem::File(file_name, file_size) => {
                if let Some(last) = stack.last() {
                    let mut dir = last.borrow_mut();
                    dir.items.push(Item::File(file_name, file_size));
                } else {
                    return Err(Error::Generic("Empty Stack".to_owned()));
                }
            }
        }
    }

    Ok(root)
}

#[derive(Debug)]
enum LineItem {
    ChangeDir(String),
    List,
    Dir(String),
    File(String, u64),
}

fn parse(s: String) -> Result<LineItem> {
    let mut it = s.as_str().split_whitespace();
    match it.next() {
        None => unreachable!("Empty Line found"),
        Some("$") => parse_command(it),
        Some(li) => parse_listing(li, it),
    }
}

fn parse_command<'a>(mut it: impl Iterator<Item = &'a str>) -> Result<LineItem> {
    match it.next() {
        Some("cd") => {
            let dir_name = it
                .next()
                .ok_or(Error::Generic("cd without dirname".to_owned()))?;
            Ok(LineItem::ChangeDir(dir_name.to_owned()))
        }
        Some("ls") => Ok(LineItem::List),
        _ => unreachable!("Unknown Command"),
    }
}

fn parse_listing<'a>(li: &'a str, mut it: impl Iterator<Item = &'a str>) -> Result<LineItem> {
    match li {
        "dir" => {
            let dir_name = it
                .next()
                .ok_or(Error::Generic("dir without dirname".to_owned()))?;
            Ok(LineItem::Dir(dir_name.to_owned()))
        }
        n => {
            let sz: u64 = n.parse()?;
            let file_name = it
                .next()
                .ok_or(Error::Generic("file without filename".to_owned()))?;
            Ok(LineItem::File(file_name.to_owned(), sz))
        }
    }
}
