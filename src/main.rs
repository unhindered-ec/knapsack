use anyhow::Context;
use ec_core::operator::selector::{best::Best, tournament::Tournament, Selector};
use ec_linear::{
    mutator::with_one_over_length::WithOneOverLength, recombinator::uniform_xo::UniformXo,
};
use knapsack::run::Run;
use knapsack::{cliff::CliffScorer, knapsack::Knapsack};
use rand::thread_rng;

// Turn some of this into CLI arguments.

fn main() -> anyhow::Result<()> {
    let knapsack = Knapsack::from_file_path("knapsacks/big.txt")
        .context("Failed to parse the knapsack file")?;

    // println!("{knapsack:?}");

    let run = Run::builder()
        .bit_length(knapsack.num_items())
        .population_size(100_000)
        .max_generations(1000)
        .scorer(CliffScorer::new(knapsack))
        .selector(Tournament::of_size::<100>())
        .recombinator(UniformXo)
        .mutator(WithOneOverLength)
        .parallel_evaluation(true)
        .build();

    let final_population = run.execute()?;

    let mut rng = thread_rng();
    let winner = Best.select(&final_population, &mut rng)?;

    println!("Best overall score: {:?}", winner.test_results);
    // println!("{winner:?}");

    Ok(())
}
