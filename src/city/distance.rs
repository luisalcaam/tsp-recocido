use super::city::Coordinates;

const EARTH_RADIUS: f64 = 6_373_000.0;


fn to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

pub fn natural_distance(u: &Coordinates, v: &Coordinates) -> f64 {
    let lat_u = to_radians(u.latitude);
    let lon_u = to_radians(u.longitude);
    let lat_v = to_radians(v.latitude);
    let lon_v = to_radians(v.longitude);

    let a = (((lat_v - lat_u) / 2.0).sin().powi(2)) +
             (lat_u.cos() * lat_v.cos() * ((lon_v - lon_u) / 2.0).sin().powi(2))      ;
    
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    EARTH_RADIUS * c
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::city::city::Coordinates;

    #[test]
    fn distance_zero() {
        // 北京
        let b = Coordinates {
            latitude: 39.92889999999999873,
            longitude: 116.3880000000000052,
        };
        assert_eq!(natural_distance(&b,&b),0.0);
    }

    #[test]
    fn distance_symmetry() {
        // 北京
        let b = Coordinates {
            latitude: 39.92889999999999873,
            longitude: 116.3880000000000052,
        };
        // CDMX
        let c = Coordinates {
            latitude: 19.43420000000000058,
            longitude: -99.1385999999999968,
        };
        assert_eq!(natural_distance(&c,&b), natural_distance(&b,&c));
        // Según Chatcito - XD - CDMX a Beijing 12500 km
        // assert_eq!(natural_distance(&c,&b),12.50);
    }    
}
