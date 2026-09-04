use std::env;

mod city;
mod input;

use input::parser::parser_file;

fn main() -> std::io::Result<()>{
    let path = env::args()
        .nth(1)
        .expect("Uso: cargo run -- <file>");
    
    let cities = parser_file(&path);

    println!("{cities:?}");
    
    Ok(())
}
