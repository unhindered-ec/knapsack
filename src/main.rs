mod cliff;
mod item;
mod knapsack;
mod run;
mod run_error;

use anyhow::Context;
use cliff::CliffScorer;
use ec_core::operator::selector::{best::Best, tournament::Tournament, Selector};
use ec_linear::{
    mutator::with_one_over_length::WithOneOverLength, recombinator::uniform_xo::UniformXo,
};
use knapsack::Knapsack;
use rand::thread_rng;
use run::Run;

// Turn some of this into CLI arguments.

fn main() -> anyhow::Result<()> {
    let knapsack = Knapsack::from_file_path("knapsacks/big.txt")
        .context("Failed to parse the knapsack file")?;

    println!("{knapsack:?}");

    let run = Run::builder()
        .bit_length(knapsack.num_items())
        .population_size(100_000)
        .max_generations(10)
        .scorer(CliffScorer::new(knapsack))
        .selector(Tournament::binary())
        .recombinator(UniformXo)
        .mutator(WithOneOverLength)
        .parallel_evaluation(false)
        .build();

    let final_population = run.execute()?;

    let mut rng = thread_rng();
    let winner = Best.select(&final_population, &mut rng);

    println!("{winner:?}");

    Ok(())
}
