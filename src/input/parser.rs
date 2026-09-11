use std::fs;
use std::io;

use crate::domain::city::CityId;

pub fn parser_file(path: &str) -> io::Result<Vec<CityId>> {
    let contents = fs::read_to_string(path)?;

    contents
        .trim()
        .split(',')
        .map(|value| {
            let id = value
                .trim()
                .parse::<u32>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

            Ok(CityId::new(id))
        })
        .collect()
}
