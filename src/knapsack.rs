use anyhow::{anyhow, Context};
use ec_core::population::Population;
use ec_linear::genome::{bitstring::Bitstring, Linear};
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
    capacity: u64,
}

impl Knapsack {
    pub fn new(items: Vec<Item>, capacity: u64) -> Self {
        Knapsack { items, capacity }
    }

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

    pub const fn capacity(&self) -> u64 {
        self.capacity
    }

    pub fn value(&self, choices: &Bitstring) -> u64 {
        self.items
            .iter()
            .zip(choices.iter())
            .filter_map(|(item, included)| included.then_some(item.value()))
            .sum()
    }

    pub fn weight(&self, choices: &Bitstring) -> u64 {
        self.items
            .iter()
            .zip(choices.iter())
            .filter_map(|(item, included)| included.then_some(item.weight()))
            .sum()
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
            .parse()?;

        Ok(Self { items, capacity })
    }
}

#[expect(clippy::unwrap_used, reason = ".unwrap() is reasonable in tests")]
#[cfg(test)]
mod tests {
    use super::Knapsack;
    use crate::item::Item;
    use ec_linear::genome::bitstring::Bitstring;
    use test_case::test_case;

    #[test]
    fn parse_from_file_path() {
        let knapsack = Knapsack::from_file_path("knapsacks/tiny.txt").unwrap();
        assert_eq!(knapsack.num_items(), 3);
        assert_eq!(knapsack.get_item(0), Some(&Item::new(1, 3, 8)));
        assert_eq!(knapsack.get_item(1), Some(&Item::new(2, 2, 8)));
        assert_eq!(knapsack.get_item(2), Some(&Item::new(3, 9, 1)));
        assert_eq!(knapsack.capacity(), 10);
    }

    #[test_case([false, false, false], 0; "choose no items")]
    #[test_case([false, true, false], 9; "choose one item")]
    #[test_case([true, false, true], 7; "choose two items")]
    fn test_values(choices: [bool; 3], expected_value: u64) {
        let knapsack = Knapsack::new(
            vec![Item::new(1, 5, 8), Item::new(2, 9, 6), Item::new(3, 2, 7)],
            100,
        );

        let choices = Bitstring::from_iter(choices);
        assert_eq!(knapsack.value(&choices), expected_value);
    }

    #[test_case([false, false, false], 0; "choose no items")]
    #[test_case([false, true, false], 6; "choose one item")]
    #[test_case([true, false, true], 15; "choose two items")]
    fn test_weights(choices: [bool; 3], expected_weight: u64) {
        let knapsack = Knapsack::new(
            vec![Item::new(1, 5, 8), Item::new(2, 9, 6), Item::new(3, 2, 7)],
            100,
        );

        let choices = Bitstring::from_iter(choices);
        assert_eq!(knapsack.weight(&choices), expected_weight);
    }
}
