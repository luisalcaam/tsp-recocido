use std::env;

mod city;
mod input;
mod database;

use input::parser::parser_file;
use database::database::load;


fn main() -> rusqlite::Result<()>{
    let path = env::args()
        .nth(1)
        .expect("Uso: cargo run -- <file>");
    
    let res = parser_file(&path);
    let city_list = res.expect("Failed to process data");



    let (cities,connections) = load("./src/database/tsp.sqlite", &city_list)?;

    
    
    Ok(())
    
}

