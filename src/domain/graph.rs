use super::city::CityIndex;

const MAX_CITIES: usize = 150;

pub struct Graph {
    adj: [[f64; MAX_CITIES]; MAX_CITIES],
    size: usize,
}

impl Graph {
    pub fn new(
        size: usize,
        connections: &[(CityIndex, CityIndex, f64)],
    ) -> Self {
        assert!(size <= MAX_CITIES);

        let mut graph = Self {
            adj: [[f64::INFINITY; MAX_CITIES]; MAX_CITIES],
            size,
        };

        for i in 0..size {
            graph.adj[i][i] = 0.0;
        }

        for &(u, v, distance) in connections {
            graph.add_edge(u, v, distance);
        }

        graph
    }

    pub fn add_edge(
        &mut self,
        u: CityIndex,
        v: CityIndex,
        distance: f64,
    ) {
        let i = u.value();
        let j = v.value();

        self.adj[i][j] = distance;
        self.adj[j][i] = distance;
    }

    pub fn weight(
        &self,
        u: CityIndex,
        v: CityIndex,
    ) -> Option<f64> {
        let weight = self.adj[u.value()][v.value()];

        if weight.is_finite() {
            Some(weight)
        } else {
            None
        }
    }

    pub fn is_connected(
        &self,
        u: CityIndex,
        v: CityIndex,
    ) -> bool {
        self.adj[u.value()][v.value()].is_finite()
    }

    pub fn size(&self) -> usize {
        self.size
    }
}
