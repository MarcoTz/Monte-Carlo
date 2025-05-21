use super::{
    report::{report_many, ChicMetrics},
    RoundRules,
};
use game::GameResult;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ChicResult {
    pub placements: Vec<String>,
    pub round_results: Vec<RoundResult>,
}

#[derive(Debug)]
pub enum RoundEnd {
    Chic,
    Normal { loser: String },
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum RoundType {
    Pickup,
    Laydown,
}

#[derive(Debug)]
pub struct RoundResult {
    pub round_type: RoundType,
    pub winner: String,
    pub starting_player: String,
    pub player_results: HashMap<String, u64>,
    pub round_end: RoundEnd,
    pub rules: RoundRules,
}

impl ChicResult {
    pub fn new() -> ChicResult {
        ChicResult {
            placements: vec![],
            round_results: vec![],
        }
    }
}

impl GameResult for ChicResult {
    type Metrics = ChicMetrics;
    fn evaluate(results: Vec<Self>) -> Self::Metrics {
        report_many(results)
    }
}

impl Default for ChicResult {
    fn default() -> ChicResult {
        ChicResult::new()
    }
}
