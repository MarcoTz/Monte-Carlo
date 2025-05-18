use rand::{prelude::SmallRng, Rng, SeedableRng};

pub struct Die {
    rng: SmallRng,
}

impl Default for Die {
    fn default() -> Die {
        Die::new()
    }
}

impl Die {
    pub fn new() -> Die {
        Die {
            rng: SmallRng::from_os_rng(),
        }
    }

    pub fn roll(&mut self) -> u8 {
        self.rng.random_range(1..=6)
    }

    pub fn roll_n(&mut self, n: usize) -> Vec<u8> {
        let mut results = vec![];
        for _ in 0..n {
            results.push(self.roll())
        }
        results
    }
}
