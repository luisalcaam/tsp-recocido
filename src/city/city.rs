pub struct Coordinates {
    pub(crate) latitude: f64,
    pub(crate) longitude: f64,
}

pub struct CityId(u32);

pub struct CityIndex(usize);

pub struct City {
    id: CityId,
    name: String,
    country: String,
    coordinates: Coordinates,
}

impl City {
    pub fn new(
        id: CityId,
        name: String,
        country: String,
        coordinates: Coordinates,
    ) -> Self {
        Self {
            id,
            name,
            country,
            coordinates,
        }
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
            coordinates,
        );

        assert_eq!(cdmx.id.0, 13);
        assert_eq!(cdmx.name, "Mexico City");
        assert_eq!(cdmx.country, "Mexico");
        assert_eq!(cdmx.coordinates.latitude, 19.4342);
        assert_eq!(cdmx.coordinates.longitude, -99.1386);
    }
}
