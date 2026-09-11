#![allow(unused)]

use std::collections::HashMap;
use rand::Rng;

use crate::solver::ThresholdAcceptingProblem;
use super::{
    city::{City,CityId,CityIndex,Connection},
    graph::Graph,
    route::Route,
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

    pub fn cost(&self, route: &Route) -> f64 {
        let mut total_distance = 0.0;
        let cities = route.cities();

        for window in cities.windows(2) {
            let u = window[0];
            let v = window[1];

            total_distance += self.graph.get_distance(u,v).unwrap_or(self.normalizer);
        }

        total_distance
    }

    pub fn is_feasible(&self, route: &Route) -> bool {
        let cities = route.cities();

        for window in cities.windows(2) {
            if !self.graph.is_connected(window[0],window[1]) {
                return false;
            }
        }
        true            
    }

    
}

impl ThresholdAcceptingProblem for Problem {
    type Solution = Route;

    fn objective_function(&self, route: &Self::Solution) -> f64 {
        self.cost(route)
    }

    fn neighbor<R: Rng + ?Sized>(&self, route: &Self::Solution, rng: &mut R) -> Self::Solution {
        //let mut rng = rand::thread_rng();
        let mut new_route = route.clone();
        let len = new_route.len();

        if len >= 2 {
            let i = rng.random_range(0..len);
            let mut j = rng.random_range(0..len);
            while i == j {
                j = rng.random_range(0..len);
            }
            new_route.reverse(i,j);
        }

        new_route
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
