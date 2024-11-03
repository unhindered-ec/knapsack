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

#[expect(
    clippy::unnecessary_wraps,
    reason = "Temporarily allowing Result wrapper"
)]
#[expect(unused_variables, reason = "Temporarily allowing unused variables")]
#[expect(unused_mut, reason = "We'll need `generation` to be mutable in a bit")]
#[expect(clippy::match_bool, reason = "I like the `match` instead of `if`")]
#[expect(clippy::todo, reason = "Todos are OK while we're working things out")]
impl<Scorer, Sel, Rec, Mut> Run<Scorer, Sel, Rec, Mut>
where
    Scorer: IndividualScorer<Bitstring, Score: Clone>,
    Sel: Selector<Vec<EcIndividual<Bitstring, Scorer::Score>>>,
    Rec: Recombinator<[Bitstring; 2], Output = Bitstring>,
    Mut: Mutator<Bitstring>,
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
                true => todo!(),
                false => generation.serial_next()?,
            }
        }

        // When we add `Generation::into_population()` we should use that here,
        // avoiding the call to `.clone()`.
        Ok(generation.population().clone())
    }
}

// These are the relevant parts of "executing" a run from the HIFF
// example in `ec-linear`.
/*
   let population = Standard
       .into_collection_generator(bit_length)
       .with_scorer(scorer)
       .into_collection_generator(population_size)
       .sample(&mut rng);

   let make_new_individual = Select::new(selector)
       .apply_twice()
       .then_map(GenomeExtractor)
       .then(Recombine::new(TwoPointXo))
       .then(Mutate::new(WithOneOverLength))
       .wrap::<GenomeScorer<_, _>>(scorer);

   let mut generation = Generation::new(make_new_individual, population);

   for generation_number in 0..num_generations {
       match run_model {
           RunModel::Serial => generation.serial_next()?,
           RunModel::Parallel => generation.par_next()?,
       }

       let best = Best.select(generation.population(), &mut rng)?;
       println!("Generation {generation_number:2} best is {best}");
   }
*/
