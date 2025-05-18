use rand::{rng, Rng};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
pub enum ScoringMethod {
    Dicke,
    Duenne,
    Dicke1,
    Dicke6,
    Dicke16,
}

impl ScoringMethod {
    pub fn rand() -> ScoringMethod {
        match rng().random_range(0..=4) {
            0 => ScoringMethod::Dicke,
            1 => ScoringMethod::Duenne,
            2 => ScoringMethod::Dicke1,
            3 => ScoringMethod::Dicke6,
            4 => ScoringMethod::Dicke16,
            _ => panic!("impossible result for rng"),
        }
    }

    pub fn score(&self, dice_val: u8) -> u64 {
        if dice_val == 1 && matches!(self, ScoringMethod::Dicke1 | ScoringMethod::Dicke16) {
            return 100;
        }

        if dice_val == 6 && matches!(self, ScoringMethod::Dicke6 | ScoringMethod::Dicke16) {
            return 60;
        }

        dice_val as u64
    }

    pub fn evaluate(&self, results: Vec<u8>) -> u64 {
        results.into_iter().map(|res| self.score(res)).sum()
    }

    pub fn compare(&self, res1: u64, res2: u64) -> Ordering {
        if matches!(self, ScoringMethod::Duenne) {
            res2.cmp(&res1)
        } else {
            res1.cmp(&res2)
        }
    }
}
