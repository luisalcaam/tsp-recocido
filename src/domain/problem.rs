use std::collections::HashMap;

use super::{
    city::{City,CityId,CityIndex,Connection},
    graph::Graph,
    //connection::Connection,
};

pub struct Problem {
    cities: Vec<City>,
    graph: Graph,
    maximum: f64,
    normalizer: f64,
}

impl Problem {
    pub fn new(
        cities: Vec<City>,
        connections: Vec<Connection>,
    ) -> Self {
        // CityId -> CityIndex
        let indices: HashMap<CityId, CityIndex> = cities
            .iter()
            .enumerate()
            .map(|(index, city)| {
                (city.id(), CityIndex(index))
            })
            .collect();

        let graph_connections: Vec<(CityIndex, CityIndex, f64)> =
            connections
            .iter()
            .map(|connection| {
                (
                    indices[&connection.city_u()],
                    indices[&connection.city_v()],
                    connection.distance(),
                )
            })
            .collect();

        let graph = Graph::new(
            cities.len(),
            &graph_connections,
        );

        let maximum = connections
            .iter()
            .map(|connection| connection.distance())
            .fold(0.0, f64::max);

        let mut distances: Vec<f64> = connections
            .iter()
            .map(|connection| connection.distance())
            .collect();

        distances.sort_unstable_by(|a, b| b.total_cmp(a));

        let normalizer: f64 = distances
            .into_iter()
            .take(cities.len().saturating_sub(1))
            .sum();

        Self {
            cities,
            graph,
            maximum,
            normalizer,
        }
    }

    pub fn maximum(&self) -> f64 {
        self.maximum
    }

    pub fn normalizer(&self) -> f64 {
        self.normalizer
    }
}

fn calculate_maximum(connections: &[Connection]) -> f64 {
    connections
        .iter()
        .map(|connection| connection.distance())
        .fold(0.0, f64::max)
}

fn calculate_normalizer(
    city_count: usize,
    connections: &[Connection],
) -> f64 {
    let mut distances: Vec<f64> = connections
        .iter()
        .map(|connection| connection.distance())
        .collect();

    distances.sort_unstable_by(|a, b| b.total_cmp(a));

    distances
        .into_iter()
        .take(city_count.saturating_sub(1))
        .sum()
}
