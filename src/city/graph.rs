//use super::distance;

pub struct Graph {
    pub(crate) adj: [[f64;150];150]
}


imple Graph {

    pub fn new(connections: Vec<Connection>) -> Self {
        for i in 0..150 {
            adj[i][i] = 0.0;
        }
        for connection in &connections {
            let v = connection.city_v;
            let u = connection.city_u;
            let dis = connection.dist;
            add_edge(v,u,dist);
        }
        
    }

    pub fn add_edge(&mut self,
                    v: CityIndex,
                    u: CityIndex,
                    dist: f64,
    ) {
        let i = v.0;
        let j = u.0;
        adj[i][j] = dis;
        adj[j][i] = dis;
    }

}




//#[cfg(test)]
//mod tests {
//    use super::*;
//    use crate::city::city::Coordinates;
//
//}
