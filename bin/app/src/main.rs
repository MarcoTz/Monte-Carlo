use chicago::{Chicago, GameConfig};
use game::Game;

const NUM_PLAYERS: u64 = 10;
const NUM_ROUNDS: u64 = 10;

fn main() {
    let res = Chicago::run_n(&GameConfig::new(NUM_PLAYERS), NUM_ROUNDS, true);
    println!();

    for game_res in res {
        for round_res in game_res.round_results {
            if round_res.winner == round_res.starting_player {
                println!(
                    "starting player won with rules {:?} and result {}",
                    round_res.rules,
                    round_res.player_results.get(&round_res.winner).unwrap()
                );
            }
        }
    }
}
