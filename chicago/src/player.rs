use crate::{die::Die, scoring::ScoringMethod, RoundRules};
use rand::{rng, Rng};

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub num_tokens: u8,
}

impl Player {
    pub fn new(name: &str) -> Player {
        Player {
            name: name.to_owned(),
            num_tokens: 0,
        }
    }

    pub fn play(&self, die: &mut Die, rules: &RoundRules, current_max: u64) -> u64 {
        let mut saved = vec![];
        let mut last_result = 0;

        for _ in 0..rules.num_rolls {
            let mut result = die.roll_n(3 - saved.len());
            let new_saved: Vec<u8> = result
                .iter()
                .filter_map(|res| (*res == 1 || *res == 6).then_some(*res))
                .collect();
            result.extend(saved.iter());
            last_result = rules.method.evaluate(result);
            if rules.method.compare(last_result, current_max).is_gt() {
                return last_result;
            }
            saved.extend(new_saved.into_iter());
        }
        last_result
    }

    pub fn play_start(&self, die: &mut Die) -> (u64, RoundRules) {
        let rules = RoundRules {
            num_rolls: rng().random_range(1..=3),
            method: ScoringMethod::rand(),
        };
        let res = self.play(die, &rules, u64::max_value());
        (res, rules)
    }
}
