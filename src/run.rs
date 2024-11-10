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
            let best = Best.select(generation.population(), &mut rng)?;
            println!("   Best score: {:?}", best.test_results);
            println!("   Entropy: {:?}", Self::entropy(generation.population()));
            match self.parallel_evaluation {
                true => generation.par_next()?,
                false => generation.serial_next()?,
            }
        }

        // When we add `Generation::into_population()` we should use that here,
        // avoiding the call to `.clone()`.
        Ok(generation.population().clone())
    }

    #[expect(
        clippy::cast_precision_loss,
        reason = "I'm happy just smashing the types for now."
    )]
    #[expect(
        clippy::as_conversions,
        reason = "I'm happy just smashing the types for now."
    )]
    fn entropy(population: &[EcIndividual<Bitstring, Scorer::Score>]) -> f64 {
        let pop_size = population.len();
        // Compute the mean of each bit position
        // Sum up (mean * log_2(mean) + (1-mean)* log_2(1- mean)) across each position.
        let bitstrings = population
            .iter()
            .map(EcIndividual::genome)
            .collect::<Vec<_>>();
        let means = (0..bitstrings[0].bits.size()).map(|index| {
            bitstrings
                .iter()
                .filter(|bitstring| bitstring.bits[index])
                .count() as f64
                / pop_size as f64
        });
        #[expect(
            clippy::suboptimal_flops,
            reason = "I'm not sure using `mul_add` buys us anything and makes it more confusing"
        )]
        means
            .map(|mean| {
                mean * (mean + f64::MIN_POSITIVE).log2()
                    + (1.0 - mean) * (1.0 - mean + f64::MIN_POSITIVE).log2()
            })
            .sum()
    }
}
