use chicago::{
    report::{report_many, write_csv},
    Chicago, GameConfig,
};
use game::Game;
use std::path::PathBuf;

const NUM_PLAYERS: u64 = 5;
const NUM_ROUNDS: u64 = 100000;

fn main() {
    let res = Chicago::run_n(&GameConfig::new(NUM_PLAYERS), NUM_ROUNDS);
    let (report_pickup, report_laydown) = report_many(res);
    write_csv(report_pickup, PathBuf::from("results/chicago.csv")).unwrap();
}
