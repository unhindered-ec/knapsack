use anyhow::{anyhow, Context};
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    str::FromStr,
};

use crate::item::Item;

#[derive(Debug)]
pub struct Knapsack {
    items: Vec<Item>,
    capacity: usize,
}

impl Knapsack {
    pub fn items(&self) -> &[Item] {
        &self.items
    }

    pub fn num_items(&self) -> usize {
        self.items.len()
    }

    pub fn get_item(&self, index: usize) -> Option<&Item> {
        self.items.get(index)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Item> {
        self.items.iter()
    }

    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn from_file_path(file_path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let file = File::open(file_path.as_ref())?;
        let reader = io::BufReader::new(file);

        // First line is number of items
        // Then one line per item with 3 integer values: Item #, item value, and item weight
        // The last line is the capacity of the knapsack
        let mut line_iter = reader.lines();
        let num_items = line_iter
            .next()
            .ok_or_else(|| anyhow!("The input file {:?} was empty", file_path.as_ref()))??
            .parse::<usize>()?;
        let items = line_iter
            .by_ref()
            .take(num_items)
            .map(|item_line_result| {
                let line_str = item_line_result
                    .context("Error reading line from knapsack specification file")?;
                Item::from_str(&line_str)
            })
            .collect::<anyhow::Result<Vec<_>>>()?;
        let capacity = line_iter
            .next()
            .ok_or_else(|| anyhow!(
                "There was no capacity line in the input file {:?}\nThis might be because the number of items was set incorrectly.",
                file_path.as_ref()
            ))??
            .parse::<usize>()?;

        Ok(Self {
            items,
            capacity,
            // Initialize fields
        })
    }
}

#[expect(clippy::unwrap_used, reason = ".unwrap() is reasonable in tests")]
#[cfg(test)]
mod test {
    use super::Knapsack;
    use crate::item::Item;

    #[test]
    fn parse_from_file_path() {
        let knapsack = Knapsack::from_file_path("knapsacks/tiny.txt").unwrap();
        assert_eq!(knapsack.num_items(), 3);
        assert_eq!(knapsack.get_item(0), Some(&Item::new(1, 3, 8)));
        assert_eq!(knapsack.get_item(1), Some(&Item::new(2, 2, 8)));
        assert_eq!(knapsack.get_item(2), Some(&Item::new(3, 9, 1)));
        assert_eq!(knapsack.capacity(), 10);
    }
}
