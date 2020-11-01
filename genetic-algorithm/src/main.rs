use rand::prelude::*;
use std::cmp::Ordering;

fn main() {
    genetic();
}

type Hypothesis = Vec<u32>;
type Fitness = f64;
type Pair = (Hypothesis, Fitness);
type Population = Vec<Pair>;
type Sizes = Vec<u32>;

fn init(p: &usize) -> (Sizes, Population) {
    let mut rng = thread_rng();
    let mut pop = Vec::with_capacity(*p);
    let mut sizes = Vec::with_capacity(100);
    for _ in 0..100 {
        sizes.push(rng.gen_range(1, 11));
    }
    for _ in 0..*p {
        let mut hypothesis = Vec::with_capacity(100);
        for _ in 0..100 {
            hypothesis.push(rng.gen_range(0, 2));
        }
        let f = fitness(&sizes, &hypothesis);
        pop.push((hypothesis, f));

    }
    (sizes, pop)
}

fn fitness(sizes: &Sizes, hypothesis: &Hypothesis) -> Fitness {
    let mut sum = 0;
    for i in 0..hypothesis.len() {
        sum += hypothesis[i] * sizes[i];
    }
    (-0.001 * (100f64 - sum as f64).powi(2)).exp()
}

fn how_close(sizes: &Sizes, hypothesis: &Hypothesis) -> i32 {
    let mut sum = 0;
    for i in 0..hypothesis.len() {
        sum += hypothesis[i] * sizes[i];
    }
    (100i32 - sum as i32).abs()
}

fn compare_hypos<'l, 'r>(
    left: &'l Pair,
    right: &'r Pair,
) -> Ordering {
    if left.1 < right.1 {
        Ordering::Less
    } else if left.1 == right.1 {
        Ordering::Equal
    } else {
        Ordering::Greater
    }
}

fn select_index(
    population: &Population,
    sum_fitness: &Fitness,
    rand_num: &f64,
    rand_index: &usize,
) -> usize {
    let mut sum = 0f64;
    let mut i = *rand_index;
    let p = population.len();
    while sum < *rand_num {
        i += 1;
        i = i % p;
        sum += population[i].1 as f64 / *sum_fitness as f64;
    }
    i
}

fn crossover(left: &Hypothesis, right: &Hypothesis) -> (Hypothesis, Hypothesis) {
    let len = left.len();
    let middle = len / 2;
    let mut l = left[0..middle].to_vec();
    l.append(&mut right.to_owned()[middle..len].to_vec());
    let mut r = right[0..middle].to_vec();
    r.append(&mut left.to_owned()[middle..len].to_vec());
    (l, r)
}

fn genetic() {
    let p = 10;
    let r = 0.5;
    let m = p / 5;
    let zero_to_one = rand::distributions::Uniform::new_inclusive(0_f64, 1_f64);
    let zero_to_p = rand::distributions::Uniform::new(0, p);
    let mut rng = thread_rng();
    let (sizes , mut population) = init(&p);
    population.sort_by(compare_hypos);

    for generation in 0..50 {
        let fittest = population.last().unwrap().clone();
        println!("Generation: {}, Off by {} Liters, Highest fitness: {}", generation, how_close(&sizes, &fittest.0), fittest.1);
        let mut next_pop = Vec::with_capacity(p);
        let sum_fitness = population.iter().fold(0f64, |x, y| x + y.1);
        // selection
        next_pop.push(fittest);  // always keep fittest individual
        let mut intersect = ((1_f64 - r) * p as f64) as usize;
        intersect += (p - intersect) % 2;  // because crossover cannot deal with an odd amount of hypos
        for _ in 0..intersect-1 {
            let i = select_index(
                &population,
                &sum_fitness,
                &zero_to_one.sample(&mut rng),
                &zero_to_p.sample(&mut rng),
            );
            next_pop.push(population[i].clone());
        }

        // crossover
        for _ in 0..(p - intersect)/2 {
            let i = select_index(
                &population,
                &sum_fitness,
                &zero_to_one.sample(&mut rng),
                &zero_to_p.sample(&mut rng),
            );
            let j = select_index(
                &population,
                &sum_fitness,
                &zero_to_one.sample(&mut rng),
                &zero_to_p.sample(&mut rng),
            );
            let cross = crossover(&population[i].0, &population[j].0);
            let (f0, f1) = (fitness(&sizes, &cross.0), fitness(&sizes, &cross.1));
            next_pop.push((cross.0, f0));
            next_pop.push((cross.1, f1));
        }

        // mutation
        for _ in 0..m {
            let i = rng.gen_range(1, p);
            let j = rng.gen_range(0, 100); 
            next_pop[i].0[j] = next_pop[i].0[j] ^ 1;
            next_pop[i].1 = fitness(&sizes, &next_pop[i].0);
        }

        // update
        population = next_pop;
        population.sort_by(compare_hypos);
    }
    println!("Result: {:?}", population.last().unwrap().0);
}
