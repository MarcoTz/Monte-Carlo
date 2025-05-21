use super::RoundRules;
use std::collections::HashMap;

#[derive(Debug)]
pub struct GameResult {
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

impl GameResult {
    pub fn new() -> GameResult {
        GameResult {
            placements: vec![],
            round_results: vec![],
        }
    }
}

impl Default for GameResult {
    fn default() -> GameResult {
        GameResult::new()
    }
}
