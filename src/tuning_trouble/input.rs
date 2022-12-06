use std::path::Path;

use crate::prelude::{Error, Result};
use crate::utils::read_lines;

pub fn read_values(file_path: &Path) -> Result<String> {
    match read_lines(file_path)?.next() {
        None => Err(Error::Generic("No Line in File".to_owned())),
        Some(r) => Ok(r?),
    }
}
