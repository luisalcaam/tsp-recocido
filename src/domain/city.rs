#[derive(Debug)]
pub struct Coordinates {
    pub(crate) latitude: f64,
    pub(crate) longitude: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CityId(u32);

impl CityId {
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    
    pub fn value(&self) -> u32 {
        self.0
    }
}

#[derive(Debug)]
pub struct Connection {
    pub(crate) city_u: CityId,
    pub(crate) city_v: CityId,
    pub(crate) distance: f64,
}

impl Connection {
    pub fn new(city_u: CityId, city_v: CityId, distance: f64) -> Self {
        Self {
            city_u,
            city_v,
            distance,
        }
    }

    pub fn city_u(&self) -> CityId {
        self.city_u
    }

    pub fn city_v(&self) -> CityId {
        self.city_v
    }
    
    pub fn distance(&self) -> f64 {
        self.distance
    }


}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CityIndex(pub usize);

impl CityIndex {
    pub fn new(index: usize) -> Self {
        Self(index)
    }

    pub fn value(&self) -> usize {
        self.0
    }
}


#[derive(Debug)]
pub struct City {
    id: CityId,
    name: String,
    country: String,
    population: u64,
    coordinates: Coordinates,
}

impl City {
    pub fn new(
        id: CityId,
        name: String,
        country: String,
        population: u64,
        coordinates: Coordinates,
    ) -> Self {
        Self {
            id,
            name,
            country,
            population,
            coordinates,
        }
    }

    pub fn id(&self) -> CityId {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn city() {
        let coordinates = Coordinates {
            latitude: 19.4342,
            longitude: -99.1386,
        };

        let cdmx = City::new(
            CityId(13),
            String::from("Mexico City"),
            String::from("Mexico"),
            8720916,
            coordinates,
        );

        assert_eq!(cdmx.id.0, 13);
        assert_eq!(cdmx.name, "Mexico City");
        assert_eq!(cdmx.country, "Mexico");
        assert_eq!(cdmx.coordinates.latitude, 19.4342);
        assert_eq!(cdmx.coordinates.longitude, -99.1386);
    }
}
