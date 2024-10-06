use ec_core::operator::selector::lexicase::Lexicase;
use ec_linear::recombinator::uniform_xo::UniformXo;

// Turn some of this into CLI arguments.

fn main() -> Result<(), RunError> {
    let knapsack = Knapsack::from_file_path("simple.txt");

    let run = Run::new(knapsack)
        .with_scorer(CliffScorer::new(knapsack))
        .with_population_size(2000)
        .with_selector(Lexicase::new(knapsack.num_items()))
        .with_mutator(UniformXo)
        .with_max_generations(10_000)
        .build()?;

    let result = run.execute();

    println!("{result}");

    Ok(())
}
