use std::env;
use rand::Rng;

mod domain;
mod input;
mod database;
mod solver;

use database::database::load;
use domain::problem::Problem;
use domain::route::Route;
use input::parser::parser_file;
use solver::ThresholdAccepting;
use domain::city::CityIndex;

fn main() -> rusqlite::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Uso: cargo run -- <file> [seed_opcional]");
        std::process::exit(1);
    }

    let path = &args[1];

    let seed: u64 = if args.len() >= 3 {
        args[2]
            .parse::<u64>()
            .expect("La semilla debe ser un entero positivo (u64)")
    } else {
        rand::rng().random()            
    };

    let city_list = parser_file(path)
        .expect("Failed to process input file");

    let (cities, connections) =
        load("./src/database/tsp.sqlite", &city_list)?;

    let city_cnt = cities.len();

    let problem = Problem::new(cities, connections);

    println!("Maximum: {}", problem.maximum());
    println!("Normalizer: {}", problem.normalizer());

    let init_indices: Vec<CityIndex> = (0..city_cnt)
        .map(CityIndex)
        .collect();

    let init_route = Route::new(init_indices);

    println!("Costo inicial: {}", problem.cost(&init_route));
    println!("Es factible inicialmente: {}", problem.is_feasible(&init_route));


    // Parámetros libres.
    let l = 5000;
    let max_attempts = l * 2;
    let epsilon = 0.001;
    let phi = 0.95;

    let solver = ThresholdAccepting::new(l, max_attempts, epsilon, phi);

    let init_t_guess = 8.0;

    println!("\nEjecutando (Seed: {})...", seed);

    let best_route = solver.solve(&problem, init_route, init_t_guess, seed);

    println!("\n--- Resultados ---");
    println!("Mejor costo encontrado: {}", problem.cost(&best_route));
    println!("Es factible: {}", problem.is_feasible(&best_route));

    print!("Path: ");
    for (i, city_idx) in best_route.cities().iter().enumerate() {
        print!("{}", city_idx.0);
        if i + 1 < best_route.len() {
            print!(", ");
        }
    }
 
    println!();

    Ok(())
}
