use std::env;

use rand::Rng;

mod domain;
mod input;
mod database;
mod solver;

use database::database::load;
use domain::city::CityIndex;
use domain::problem::Problem;
use domain::route::Route;
use input::config::Config;
use input::parser::parser_file;
use solver::ThresholdAccepting;

fn main() -> rusqlite::Result<()> {
    let args: Vec<String> = env::args().collect();



    if args.len() < 2 || args.len() > 3 {
        eprintln!(
            "Uso: cargo run -- <file.tsp>\n  cargo run -- <file.tsp> <config.toml>"
        );
        std::process::exit(1);
    }

    let tsp_path = &args[1];


    let config = if args.len() == 3 {
        Config::from_file(&args[2])
            .expect("No se pudo leer el archivo de configuración")
    } else {
        Config::default()
    };


    let seed = config
        .seed
        .unwrap_or_else(|| rand::rng().random());


    let city_list = parser_file(tsp_path)
        .expect("Failed to process input file");

    let (cities, connections) =
        load("./src/database/tsp.sqlite", &city_list)?;

    let city_cnt = cities.len();

    let problem = Problem::new(cities, connections);

    println!("Maximum: {}", problem.maximum());
    println!("Normalizer: {}", problem.normalizer());


    let init_indices: Vec<CityIndex> = (0..city_cnt)
        .map(CityIndex::new)
        .collect();

    let init_route = Route::new(init_indices);



    let solver = ThresholdAccepting::new(
        config.l,
        config.max_attempts,
        config.epsilon,
        config.phi,
        config.p_target,
        config.n_samples,
        config.epsilon_p,
    );

    let init_t_guess = config.initial_t_guess;

    println!("Parámetros");
    println!("L: {}", config.l);
    println!("Max attempts: {}", config.max_attempts);
    println!("Epsilon: {}", config.epsilon);
    println!("Phi: {}", config.phi);
    println!("Seed: {}", seed);

    println!("Solución inicial");
    println!("Costo inicial: {}", problem.cost(&init_route));
    println!(
        "Evaluación inicial: {}",
        problem.cost(&init_route) / problem.normalizer()
    );
    println!(
        "Es factible inicialmente: {}",
        problem.is_feasible(&init_route)
    );

    

    println!("\nEjecutando...");

    let best_route =
        solver.solve(&problem, init_route, init_t_guess, seed);

    
    println!("\n--- Resultados ---");

    let best_cost = problem.cost(&best_route);
    let best_evaluation = best_cost / problem.normalizer();

    println!("Mejor costo encontrado: {}", best_cost);
    println!("Evaluación: {}", best_evaluation);
    println!(
        "Es factible: {}",
        problem.is_feasible(&best_route)
    );
    println!("Seed: {}", seed);



    
    print!("Path: ");

    for (i, city_idx) in best_route.cities().iter().enumerate() {
        print!("{}", problem.city_id(*city_idx).value());
        if i + 1 < best_route.len() {
            print!(",");
        }
    }

    println!();

    Ok(())
}
