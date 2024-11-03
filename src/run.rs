use ec_core::{
    distributions::collection::ConvertToCollectionGenerator,
    generation::Generation,
    individual::{
        ec::{EcIndividual, WithScorer},
        scorer::Scorer as IndividualScorer,
    },
    operator::{
        genome_extractor::GenomeExtractor,
        genome_scorer::GenomeScorer,
        mutator::{Mutate, Mutator},
        recombinator::{Recombinator, Recombine},
        selector::{Select, Selector},
        Composable,
    },
};
use ec_linear::genome::bitstring::Bitstring;
use rand::{
    distr::{Distribution, Standard},
    thread_rng,
};
use typed_builder::TypedBuilder;

// TODO: What if we want to allow people to specify either a recombinator or a mutator
// or both? (They have to provide at least one, but they don't have to provide both.)

#[derive(TypedBuilder)]
pub struct Run<Scorer, Sel, Rec, Mut> {
    bit_length: usize,

    population_size: usize,

    max_generations: usize,

    parallel_evaluation: bool,

    scorer: Scorer,
    selector: Sel,
    recombinator: Rec,
    mutator: Mut,
}

#[expect(clippy::match_bool, reason = "I like the `match` instead of `if`")]
impl<Scorer, Sel, Rec, Mut> Run<Scorer, Sel, Rec, Mut>
where
    Scorer: IndividualScorer<Bitstring, Score: Clone + Send + Sync> + Send + Sync,
    Sel: Selector<Vec<EcIndividual<Bitstring, Scorer::Score>>> + Send + Sync,
    Rec: Recombinator<[Bitstring; 2], Output = Bitstring> + Send + Sync,
    Mut: Mutator<Bitstring> + Send + Sync,
{
    pub fn execute(self) -> anyhow::Result<Vec<EcIndividual<Bitstring, Scorer::Score>>> {
        let mut rng = thread_rng();

        // Create the initial population for the run
        let population =
            // `Standard` can be used to generate random booleans, which will be
            // used below to generate the `Bitstring`s.
            Standard
            // Generate a `Bitstring` of length `self.bit_length`
            .into_collection_generator(self.bit_length)
            // Adds a scorer to the `Bitstring`, creating an `Individual`
            .with_scorer(&self.scorer)
            // Create a `Population` of `self.population_size` `Individual`s
            .into_collection_generator(self.population_size)
            // Actually sample the distribution to get the initial population.
            .sample(&mut rng);

        // Make an operator that takes a population and generates a new (child) individual.
        let child_maker =
            // Select a random individual to be a parent
            Select::new(self.selector)
            // Select twice, generating two parents
            .apply_twice()
            // Extract the genomes from those two parents, yielding a pair of genomes (`Bitstring`s)
            .then_map(GenomeExtractor)
            // Combine those genomes (`Bitstrings`s) into a new child genome
            .then(Recombine::new(self.recombinator))
            // Mutate the resulting genome (`Bitstring`)
            .then(Mutate::new(self.mutator))
            // Score the resulting mutated genome to generate an `Individual`
            .wrap::<GenomeScorer<_, _>>(&self.scorer);

        let mut generation = Generation::new(child_maker, population);

        for generation_number in 0..self.max_generations {
            println!("Generation {generation_number}");
            match self.parallel_evaluation {
                true => generation.par_next()?,
                false => generation.serial_next()?,
            }
        }

        // When we add `Generation::into_population()` we should use that here,
        // avoiding the call to `.clone()`.
        Ok(generation.population().clone())
    }
}
