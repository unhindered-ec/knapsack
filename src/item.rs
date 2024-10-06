use std::{num::ParseIntError, str::FromStr};

#[derive(Debug, thiserror::Error)]
pub enum ItemParseError {
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),

    #[error("The item specification line '{0}' should have had {1} whitespace separated fields")]
    IncorrectNumberOfFields(String, usize),
}
pub struct Item {
    id: usize,
    value: usize,
    weight: usize,
}

impl Item {
    const fn new(id: usize, value: usize, weight: usize) -> Self {
        Self { id, value, weight }
    }
}

impl FromStr for Item {
    type Err = ItemParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .split_ascii_whitespace()
            .map(usize::from_str)
            .collect::<Result<Vec<usize>, _>>()?;
        if values.len() != 3 {
            return Err(ItemParseError::IncorrectNumberOfFields(
                s.to_owned(),
                values.len(),
            ));
        }
        Ok(Item::new(values[0], values[1], values[2]))
    }
}
