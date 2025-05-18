pub mod die;
pub mod player;
pub mod scoring;

use die::Die;
use player::Player;
use scoring::ScoringMethod;

pub struct RoundResult {
    pub loser: usize,
    pub winner: usize,
}

#[derive(Debug)]
pub struct RoundRules {
    method: ScoringMethod,
    num_rolls: u8,
}

pub fn play_game(num_players: u8) {
    let mut die = Die::new();
    let mut players: Vec<Player> = (0..num_players)
        .map(|i| Player::new(&format!("player_{i}")))
        .collect();
    play_pickup(&mut players, num_players + 1, &mut die);
    println!("");
    println!("starting removing tokens");
    println!(
        "current tokens: {}",
        players
            .iter()
            .map(|pl| format!("{}: {}", pl.name, pl.num_tokens))
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!("");
    players = players
        .into_iter()
        .filter(|pl| pl.num_tokens != 0)
        .collect();

    play_laydown(&mut players, &mut die);

    println!("");
    println!("Round has ended, {} lost", players[0].name);
}

pub fn play_round(players: &[Player], starting_ind: usize, die: &mut Die) -> RoundResult {
    let mut loser_ind;
    let mut winner_ind;
    let mut loser_score;
    let mut winner_score;

    let starting_player = &players[starting_ind];
    let (starting_score, rules) = starting_player.play_start(die);
    println!("round rules: {rules:?}");
    loser_ind = starting_ind;
    winner_ind = starting_ind;
    loser_score = starting_score;
    winner_score = starting_score;

    println!("result for starting player: {}", starting_score);

    for i in 1..=(players.len() - 1) {
        let next_ind = (starting_ind + i) % players.len();
        let next_player = &players[next_ind];
        let next_score = next_player.play(die, &rules, winner_score);

        if rules.method.compare(next_score, loser_score).is_le() {
            loser_score = next_score;
            loser_ind = next_ind;
        }

        if rules.method.compare(next_score, winner_score).is_gt() {
            winner_score = next_score;
            winner_ind = next_ind;
        }

        println!("result for player {next_ind}: {}", next_score);
    }
    RoundResult {
        loser: loser_ind,
        winner: winner_ind,
    }
}

pub fn next_start(players: &[Player], die: &mut Die) -> usize {
    let max_tokens = players.iter().map(|pl| pl.num_tokens).max().unwrap();
    let mut candidates: Vec<usize> = (0..players.len())
        .filter(|i| players[*i].num_tokens == max_tokens)
        .collect();
    while candidates.len() > 1 {
        let results: Vec<(usize, u8)> = candidates.iter().map(|ind| (*ind, die.roll())).collect();
        let max_res = results
            .iter()
            .max_by(|(_, res1), (_, res2)| res1.cmp(&res2))
            .unwrap()
            .1;
        candidates = results
            .into_iter()
            .filter_map(|(ind, res)| (res == max_res).then_some(ind))
            .collect();
    }
    candidates[0]
}

pub fn play_pickup(players: &mut [Player], num_tokens: u8, die: &mut Die) {
    let mut start_ind = 0;

    for _ in 0..num_tokens {
        println!("starting player: {}", players[start_ind].name);
        let result = play_round(&players, start_ind, die);
        let loser = players.get_mut(result.loser).unwrap();
        loser.num_tokens += 1;
        println!(
            "loser is {}, now has {} tokens",
            loser.name, loser.num_tokens
        );
        start_ind = result.loser;
    }
}

pub fn play_laydown(players: &mut Vec<Player>, die: &mut Die) {
    let num_tokens: u8 = players.iter().map(|pl| pl.num_tokens).sum();
    let mut start_ind;

    for _ in 0..(num_tokens - 1) {
        start_ind = next_start(&players, die);
        let result = play_round(&players, start_ind, die);
        let winner = players.get_mut(result.winner).unwrap();
        winner.num_tokens -= 1;
        println!(
            "winner is {}, now has {} tokens",
            winner.name, winner.num_tokens
        );
        if winner.num_tokens == 0 {
            println!("{} finished the round", winner.name);
            players.remove(result.winner);
        }
    }
}
