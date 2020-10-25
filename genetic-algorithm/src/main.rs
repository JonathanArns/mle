use rand::prelude::*;
use std::cmp::Ordering;

fn main() {
    genetic(1000);
}

type Bitstring = u32;
type Fitness = i32;
type Hypothesis = (Bitstring, Fitness);
type Population = Vec<Hypothesis>;

fn init(p: usize) -> (Bitstring, Population) {
    let mut rng = thread_rng();
    let target = rng.gen();
    let mut pop = Vec::with_capacity(p);
    for _ in 0..p {
        let hypothesis = rng.gen();
        pop.push((hypothesis, fitness(target, hypothesis)));
    }
    (target, pop)
}

fn fitness(target: Bitstring, actual: Bitstring) -> Fitness {
    let mut count = 0;
    for i in 0..32 {
        if (target >> i) & 1 == (actual >> i) & 1 {
            count += 1;
        }
    }
    count
}

fn compare_hypos<'l, 'r>(
    left: &'l Hypothesis,
    right: &'r Hypothesis,
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
    sum_fitness: &i32,
    rand_num: &f64,
    rand_index: &usize,
) -> usize {
    let mut sum = 0_f64;
    let mut i = *rand_index;
    let p = population.len();
    while sum < *rand_num {
        i += 1;
        i = i % p;
        sum += population[i].1 as f64 / *sum_fitness as f64;
    }
    i
}

fn crossover(left: &Bitstring, right: &Bitstring) -> (Bitstring, Bitstring) {
    let mask = 0x_ffff0000_u32;
    let l = (left & mask) ^ (right & !mask);
    let r = (right & mask) ^ (left & !mask);
    (l, r)
}

fn genetic(p: usize) {
    let r = 0.3;
    let m = p / 3;
    let fitness_threshold = 32;
    let zero_to_one = rand::distributions::Uniform::new_inclusive(0_f64, 1_f64);
    let zero_to_p = rand::distributions::Uniform::new(0, p);
    let mut rng = thread_rng();
    let (target, mut population) = init(p);
    population.sort_by(compare_hypos);

    while population.last().unwrap().1 < fitness_threshold {
        println!("{:?}", population.last().unwrap());
        let mut next_pop = Vec::with_capacity(p);
        let sum_fitness = population.iter().fold(0, |x, y| x + y.1);
        // selection
        let mut intersect = ((1_f64 - r) as usize) * p;
        intersect += (p - intersect) % 2;  // because crossover cannot deal with an odd amount of hypos
        for _ in 0..intersect {
            let i = select_index(
                &population,
                &sum_fitness,
                &zero_to_one.sample(&mut rng),
                &zero_to_p.sample(&mut rng),
            );
            next_pop.push(population[i]);
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
            next_pop.push((cross.0, fitness(target, cross.0)));
            next_pop.push((cross.1, fitness(target, cross.1)));
        }

        // mutation
        for _ in 0..m {
            let i = zero_to_p.sample(&mut rng);
            let old = next_pop.remove(i);
            let mutated = old.0 ^ (1_u32 << rng.gen_range(0_u32 , 32_u32 ));
            next_pop.push((mutated, fitness(target, mutated)));
        }

        // update
        population = next_pop;
        population.sort_by(compare_hypos);
    }
}
