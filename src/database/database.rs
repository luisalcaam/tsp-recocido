use rusqlite::{Connection, Result};

use crate::domain::city::{City, CityId, Coordinates};
use crate::domain::city::Connection as CityConnection;

pub fn load(
    path: &str,
    cities: &[CityId]
) -> Result<(Vec<City>, Vec<CityConnection>)> {
    let db = Connection::open(path)?;

    let list_cities = load_cities(&db, cities)?;
    
    let connections = load_connections(&db, cities)?;

    Ok((list_cities, connections))
}

fn load_cities(
   db: &Connection,
   cities: &[CityId]
) -> Result<Vec<City>> {

    if cities.is_empty() {
       return Ok(Vec::new());
    }

    let placeholders = std::iter::repeat("?")
        .take(cities.len())
        .collect::<Vec<_>>()
        .join(", ");

    
    
    let query = format!(
        "
        SELECT
            id,
            name,
            country,
            population,
            latitude,
            longitude
        FROM cities
        WHERE id IN ({})
        ",
        placeholders
    );

    let mut statement = db.prepare(&query)?;

    let params: Vec<u32> = cities
        .iter()
        .map(|city_id| city_id.value())
        .collect();

    let cities = statement
        .query_map(rusqlite::params_from_iter(params), |row| {
            let id: u32 = row.get(0)?;
            let name: String = row.get(1)?;
            let country: String = row.get(2)?;
            let population: u64 = row.get(3)?;
            let latitude: f64 = row.get(4)?;
            let longitude: f64 = row.get(5)?;

            Ok(City::new(
                CityId::new(id),
                name,
                country,
                population,
                Coordinates {
                    latitude,
                    longitude,
                },
            ))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(cities)
}

fn load_connections(
    db: &Connection,
    cities: &[CityId]
) -> Result<Vec<CityConnection>> {
    if cities.is_empty() {
        return Ok(Vec::new());
    }

    let placeholders = std::iter::repeat("?")
        .take(cities.len())
        .collect::<Vec<_>>()
        .join(", ");

    let query = format!(
        "
        SELECT
            id_city_1,
            id_city_2,
            distance
        FROM connections
        WHERE id_city_1 IN ({})
          AND id_city_2 IN ({})
        ",
        placeholders,
        placeholders,
    );

    let mut statement = db.prepare(&query)?;

    let params: Vec<u32> = cities
        .iter()
        .map(|city_id| city_id.value())
        .chain(cities.iter().map(|city_id| city_id.value()))
        .collect();

    let connections = statement
        .query_map(rusqlite::params_from_iter(params), |row| {
            let city_u: u32 = row.get(0)?;
            let city_v: u32 = row.get(1)?;
            let distance: f64 = row.get(2)?;

            Ok(CityConnection::new(
                CityId::new(city_u),
                CityId::new(city_v),
                distance,
            ))
        })?
        .collect::<Result<Vec<_>, _>>()?;   

    Ok(connections)
}
