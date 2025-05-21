use super::{
    results::{GameResult, RoundType},
    scoring::ScoringMethod,
};
use csv::WriterBuilder;
use std::{fs::OpenOptions, path::PathBuf};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct StartResult {
    round_type: RoundType,
    num_games: u64,
    wins_duenne: u64,
    wins_dicke: u64,
    wins_dicke1: u64,
    wins_dicke6: u64,
    wins_dicke16: u64,
}

impl StartResult {
    pub fn new(ty: RoundType) -> StartResult {
        StartResult {
            round_type: ty,
            num_games: 0,
            wins_duenne: 0,
            wins_dicke: 0,
            wins_dicke1: 0,
            wins_dicke6: 0,
            wins_dicke16: 0,
        }
    }
}

impl StartResult {
    pub fn combine(self, other: StartResult) -> StartResult {
        StartResult {
            round_type: self.round_type,
            num_games: self.num_games + other.num_games,
            wins_duenne: self.wins_duenne + other.wins_duenne,
            wins_dicke: self.wins_dicke + other.wins_dicke,
            wins_dicke1: self.wins_dicke1 + other.wins_dicke1,
            wins_dicke6: self.wins_dicke6 + other.wins_dicke6,
            wins_dicke16: self.wins_dicke16 + other.wins_dicke16,
        }
    }
}

pub fn report_many(results: Vec<GameResult>) -> (StartResult, StartResult) {
    let mut report_pickup = StartResult::new(RoundType::Pickup);
    let mut report_laydown = StartResult::new(RoundType::Laydown);
    for result in results {
        report_pickup.num_games += 1;
        report_laydown.num_games += 1;
        let (next_pickup, next_laydown) = report_game(result);
        report_pickup = report_pickup.combine(next_pickup);
        report_laydown = report_laydown.combine(next_laydown);
    }
    (report_pickup, report_laydown)
}

pub fn report_game(result: GameResult) -> (StartResult, StartResult) {
    let mut wins_pickup = StartResult::new(RoundType::Pickup);
    let mut wins_laydown = StartResult::new(RoundType::Laydown);

    for round_res in result.round_results {
        let winner = round_res.winner;
        let start = round_res.starting_player;
        if winner != start {
            continue;
        }

        let current_results = match round_res.round_type {
            RoundType::Laydown => &mut wins_laydown,
            RoundType::Pickup => &mut wins_pickup,
        };

        match round_res.rules.method {
            ScoringMethod::Duenne => current_results.wins_duenne += 1,
            ScoringMethod::Dicke => current_results.wins_dicke += 1,
            ScoringMethod::Dicke1 => current_results.wins_dicke1 += 1,
            ScoringMethod::Dicke6 => current_results.wins_dicke6 += 1,
            ScoringMethod::Dicke16 => current_results.wins_dicke16 += 1,
        }
    }
    (wins_pickup, wins_laydown)
}

pub fn write_csv(data: StartResult, out_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let write_headers = !out_path.exists();
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(out_path)?;
    let mut wtr = WriterBuilder::new()
        .has_headers(write_headers)
        .from_writer(file);
    wtr.serialize(data)?;
    wtr.flush()?;
    Ok(())
}
