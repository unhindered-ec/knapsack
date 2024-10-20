use anyhow::ensure;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct Item {
    id: u64,
    value: u64,
    weight: u64,
}

impl Item {
    pub const fn new(id: u64, value: u64, weight: u64) -> Self {
        Self { id, value, weight }
    }

    pub const fn id(&self) -> u64 {
        self.id
    }

    pub const fn value(&self) -> u64 {
        self.value
    }

    pub const fn weight(&self) -> u64 {
        self.weight
    }
}

impl FromStr for Item {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .split_ascii_whitespace()
            .map(FromStr::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        ensure!(
            values.len() == 3,
            "The item specification line should have had 3 whitespace separated fields"
        );
        Ok(Self::new(values[0], values[1], values[2]))
    }
}
