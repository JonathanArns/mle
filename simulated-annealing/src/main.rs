// Matrikel Nr.: 1811609
use rand::prelude::*;

fn main() {
    let (world, mut path) = create_world(100);
    println!("{:?}", world);
    optimize(&world, &mut path);
}

type World = Vec<Vec<usize>>;
type Path = Vec<usize>;

fn create_world(n: usize) -> (World, Path) {
    let mut rng = rand::thread_rng();
    let mut world = Vec::with_capacity(n);
    for i in 0..n {
        world.push(Vec::with_capacity(n));
        for _ in 0..i {
            world[i].push(rng.gen_range(1, 100));
        }
        world[i].push(0);
    }
    for i in 0..n {
        for j in i+1..n {
            let x = world[j][i];
            world[i].push(x);
        }
    }
    (world, (0..n).collect())
}

fn get_distance(world: &World, path: &Path) -> usize {
    let mut distance = 0;
    for x in 0..path.len()-1 {
        distance += world[path[x]][path[x+1]];
    }
    distance + world[path[0]][path[path.len()-1]]
}

fn get_fitness(world: &World, path: &Path) -> i32 {
    -(get_distance(world, path) as i32)
}

fn swap(path: &mut Path, x: usize, y: usize) {
    let tmp = path[x];
    path[x] = path[y];
    path[y] = tmp;
}

fn swap_random(path: &mut Path) -> (usize, usize) {
    let mut rng = rand::thread_rng();
    let (x, y) = (rng.gen_range(0, path.len()), rng.gen_range(0, path.len()));
    swap(path, x, y);
    (x, y)
}

fn optimize(world: &World, path: &mut Path) {
    let mut rng = rand::thread_rng();
    let range = rand::distributions::Uniform::new_inclusive(0_f64, 1_f64);
    let epsilon = 0.0001;
    let mut temp = 10.0;
    let mut fitness = get_fitness(world, path);
    while temp > epsilon {
        let (x, y) = swap_random(path);
        let new_fitness = get_fitness(world, path);
        if new_fitness > fitness {
            fitness = new_fitness;
            println!("Temperature: {}  Distance: {}  Reise: {:?}", temp, get_distance(world, path), path);
        } else {
            let probability = ((new_fitness as f64 - fitness as f64)/temp).exp();
            println!("Temperature: {}  Probability: {}", temp, probability);
            if range.sample(&mut rng) < probability {
                fitness = new_fitness;
                println!("Temperature: {}  Distance: {}  Reise: {:?}", temp, get_distance(world, path), path);
            } else {
                swap(path, x, y);
            }
        }
        temp -= epsilon;
    }
    println!("(Ergebnis) Distance: {}  Reise: {:?}", get_distance(world, path), path);
}
