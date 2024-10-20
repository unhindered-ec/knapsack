use crate::knapsack::{self, Knapsack};

pub struct Run {
    knapsack: Knapsack, // This needs to become more generic
}

impl Run {
    pub fn new(knapsack: Knapsack) -> Self {
        Run { knapsack }
    }
}
