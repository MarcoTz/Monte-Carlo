use chicago::play_game;

const NUM_PLAYERS: u8 = 10;

fn main() {
    let res = play_game(NUM_PLAYERS);
    println!("placements: {:?}\n{}", res.placements, res.placements.len())
}
