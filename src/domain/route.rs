#![allow(unused)]
use super::city::CityIndex;


#[derive(Debug, Clone)]
pub struct Route {
    cities: Vec<CityIndex>,
}

impl Route {
    pub fn new(cities: Vec<CityIndex>) -> Self {
        Self { cities }
    }

    pub fn len(&self) -> usize {
        self.cities.len()
    }

    pub fn city_at(&self, index: usize) -> CityIndex {
        self.cities[index]
    }

    pub fn swap(&mut self, i: usize, j: usize) {
        self.cities.swap(i, j);
    }

    pub fn cities(&self) -> &[CityIndex] {
        &self.cities
    }

    pub fn reverse(&mut self, mut i: usize, mut j: usize) {
        if i > j {
            std::mem::swap(&mut i, &mut j);
        }
        self.cities[i..=j].reverse();
    }
}
