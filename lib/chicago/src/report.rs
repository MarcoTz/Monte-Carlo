use super::{
    results::{ChicResult, RoundType},
    scoring::ScoringMethod,
};

#[derive(Default, Debug, serde::Serialize, serde::Deserialize)]
pub struct ChicMetrics {
    num_games: u64,
    wins_duenne_pickup: u64,
    wins_dicke_pickup: u64,
    wins_dicke1_pickup: u64,
    wins_dicke6_pickup: u64,
    wins_dicke16_pickup: u64,
    wins_duenne_laydown: u64,
    wins_dicke_laydown: u64,
    wins_dicke1_laydown: u64,
    wins_dicke6_laydown: u64,
    wins_dicke16_laydown: u64,
}

impl ChicMetrics {
    pub fn combine(self, other: ChicMetrics) -> ChicMetrics {
        ChicMetrics {
            num_games: self.num_games + other.num_games,
            wins_duenne_pickup: self.wins_duenne_pickup + other.wins_duenne_pickup,
            wins_dicke_pickup: self.wins_dicke_pickup + other.wins_dicke_pickup,
            wins_dicke1_pickup: self.wins_dicke1_pickup + other.wins_dicke1_pickup,
            wins_dicke6_pickup: self.wins_dicke6_pickup + other.wins_dicke6_pickup,
            wins_dicke16_pickup: self.wins_dicke16_pickup + other.wins_dicke16_pickup,
            wins_duenne_laydown: self.wins_duenne_laydown + other.wins_duenne_laydown,
            wins_dicke_laydown: self.wins_dicke_laydown + other.wins_dicke_laydown,
            wins_dicke1_laydown: self.wins_dicke1_laydown + other.wins_dicke1_laydown,
            wins_dicke6_laydown: self.wins_dicke6_laydown + other.wins_dicke6_laydown,
            wins_dicke16_laydown: self.wins_dicke16_laydown + other.wins_dicke16_laydown,
        }
    }
}

pub fn report_many(results: Vec<ChicResult>) -> ChicMetrics {
    let mut metrics = ChicMetrics {
        num_games: results.len() as u64,
        ..Default::default()
    };
    for result in results {
        let next_metric = report_game(result);
        metrics = metrics.combine(next_metric);
    }
    metrics
}

pub fn report_game(result: ChicResult) -> ChicMetrics {
    let mut report = ChicMetrics::default();

    for round_res in result.round_results {
        let winner = round_res.winner;
        let start = round_res.starting_player;
        if winner != start {
            continue;
        }

        match (round_res.round_type, round_res.rules.method) {
            (RoundType::Laydown, ScoringMethod::Duenne) => report.wins_duenne_laydown += 1,
            (RoundType::Laydown, ScoringMethod::Dicke) => report.wins_dicke_laydown += 1,
            (RoundType::Laydown, ScoringMethod::Dicke1) => report.wins_dicke1_laydown += 1,
            (RoundType::Laydown, ScoringMethod::Dicke6) => report.wins_dicke6_laydown += 1,
            (RoundType::Laydown, ScoringMethod::Dicke16) => report.wins_dicke16_laydown += 1,
            (RoundType::Pickup, ScoringMethod::Duenne) => report.wins_duenne_pickup += 1,
            (RoundType::Pickup, ScoringMethod::Dicke) => report.wins_dicke_pickup += 1,
            (RoundType::Pickup, ScoringMethod::Dicke1) => report.wins_dicke1_pickup += 1,
            (RoundType::Pickup, ScoringMethod::Dicke6) => report.wins_dicke6_pickup += 1,
            (RoundType::Pickup, ScoringMethod::Dicke16) => report.wins_dicke16_pickup += 1,
        }
    }
    report
}
