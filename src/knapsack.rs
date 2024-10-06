use std::{
    fs::File,
    io::{self, BufRead},
    num::ParseIntError,
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::item::{Item, ItemParseError};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("Tried to parse empty file")]
    EmptyFile(PathBuf),

    #[error(transparent)]
    IllegalNumberOfItems(#[from] ParseIntError),

    #[error(transparent)]
    ItemParse(#[from] ItemParseError),

    #[error("No capacity line")]
    NoCapacity,
}

pub struct Knapsack {
    items: Vec<Item>,
    capacity: usize,
}

impl Knapsack {
    pub fn from_file_path(file_path: impl AsRef<Path>) -> Result<Self, self::Error> {
        let file = File::open(file_path.as_ref())?;
        let reader = io::BufReader::new(file);

        // First line is number of items
        // Then one line per item with 3 integer values: Item #, item value, and item weight
        // The last line is the capacity of the knapsack
        let mut line_iter = reader.lines();
        let num_items = line_iter
            .next()
            .ok_or(Error::EmptyFile(file_path.as_ref().to_owned()))??
            .parse::<usize>()?;
        let items = line_iter
            .by_ref()
            .take(num_items)
            // The items from `take` are `Result` and we have to deal with that
            // before (or during) `map`.
            .map(Item::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        let capacity = line_iter
            .next()
            .ok_or(Error::NoCapacity)??
            .parse::<usize>()?;

        Ok(Knapsack {
            items,
            capacity,
            // Initialize fields
        })
    }
}
