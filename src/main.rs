mod cliff_scorer;
mod item;
mod knapsack;
mod run;
mod run_error;

use cliff_scorer::CliffScorer;
use ec_core::operator::selector::tournament::Tournament;
use ec_linear::mutator::with_one_over_length::WithOneOverLength;
use knapsack::Knapsack;
use run::Run;

// Turn some of this into CLI arguments.

fn main() -> anyhow::Result<()> {
    let knapsack = Knapsack::from_file_path("knapsacks/tiny.txt")?;

    println!("{knapsack:?}");

    let run = Run::new(knapsack)
        .with_scorer(CliffScorer::new(knapsack))
        .with_population_size(2000)
        .with_selector(Tournament::binary())
        .with_mutator(WithOneOverLength)
        .with_max_generations(10_000)
        .build()?;

    let result = run.execute();

    println!("{result}");

    Ok(())
}
