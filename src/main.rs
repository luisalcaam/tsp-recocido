use std::env;

mod domain;
mod input;
mod database;

use database::database::load;
use domain::problem::Problem;
use input::parser::parser_file;

fn main() -> rusqlite::Result<()> {
    let path = env::args()
        .nth(1)
        .expect("Uso: cargo run -- <file>");

 
    let city_list = parser_file(&path)
        .expect("Failed to process input file");

 
    let (cities, connections) =
        load("./src/database/tsp.sqlite", &city_list)?;

 
    let problem = Problem::new(cities, connections);

 
    println!("Maximum: {}", problem.maximum());
    println!("Normalizer: {}", problem.normalizer());

    Ok(())
}
