use ec_core::{individual::scorer::Scorer, test_results::Score};
use ec_linear::genome::bitstring::Bitstring;

use crate::knapsack::Knapsack;

struct CliffScorer {
    knapsack: Knapsack,
}

impl CliffScorer {
    fn new(knapsack: Knapsack) -> Self {
        Self { knapsack }
    }
}

impl Scorer<Bitstring> for CliffScorer {
    type Score = anyhow::Result<Score<i64>>;

    fn score(&self, genome: &Bitstring) -> Self::Score {
        todo!()
    }
}
