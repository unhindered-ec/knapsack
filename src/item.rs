use anyhow::ensure;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct Item {
    id: usize,
    value: usize,
    weight: usize,
}

impl Item {
    pub const fn new(id: usize, value: usize, weight: usize) -> Self {
        Self { id, value, weight }
    }

    pub const fn id(&self) -> usize {
        self.id
    }

    pub const fn value(&self) -> usize {
        self.value
    }

    pub const fn weight(&self) -> usize {
        self.weight
    }
}

impl FromStr for Item {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .split_ascii_whitespace()
            .map(usize::from_str)
            .collect::<Result<Vec<usize>, _>>()?;
        ensure!(
            values.len() == 3,
            "The item specification line should have had 3 whitespace separated fields"
        );
        Ok(Self::new(values[0], values[1], values[2]))
    }
}
