use std::fs;
use std::path::Path;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    pub l: usize,
    pub max_attempts: usize,

    pub epsilon: f64,
    pub phi: f64,

    pub p_target: f64,
    pub n_samples: usize,
    pub epsilon_p: f64,

    pub initial_t_guess: f64,

    pub seed: Option<u64>,
}

impl Default for Config {
    fn default() -> Self {
        let l = 2000;

        Self {
            l,
            max_attempts: l * 2,

            epsilon: 0.0000001,
            phi: 0.95,

            p_target: 0.90,
            n_samples: 100,
            epsilon_p: 0.01,

            initial_t_guess: 8.0,

            seed: None,
        }
    }
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let config = toml::from_str(&contents)?;
        Ok(config)
    }
}
