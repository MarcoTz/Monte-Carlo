use chicago::{Chicago, GameConfig};
use game::{write_csv, Game, GameResult};
use std::path::PathBuf;

const NUM_PLAYERS: u64 = 5;
const NUM_ROUNDS: u64 = 100000;

fn main() {
    let res = Chicago::run_n(&GameConfig::new(NUM_PLAYERS), NUM_ROUNDS);
    let metrics = GameResult::evaluate(res);
    write_csv(metrics, PathBuf::from("results/chicago.csv")).unwrap();
}
