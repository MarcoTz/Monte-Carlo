pub mod die;
pub mod player;
pub mod scoring;

use die::Die;
use player::Player;
use scoring::ScoringMethod;

pub struct GameResult {
    pub placements: Vec<String>,
}

pub struct RoundResult {
    pub loser: usize,
    pub winner: usize,
}

#[derive(Debug)]
pub struct RoundRules {
    method: ScoringMethod,
    num_rolls: u8,
}

pub fn play_game(num_players: u8) -> GameResult {
    let mut placements = vec![];
    let mut die = Die::new();
    let mut players: Vec<Player> = (0..num_players)
        .map(|i| Player::new(&format!("player_{i}")))
        .collect();
    play_pickup(&mut players, num_players + 1, &mut die, &mut placements);
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
        .enumerate()
        .filter_map(|(ind, pl)| {
            if pl.num_tokens == 0 {
                placements.push(pl.name.clone());
                None
            } else {
                Some(pl)
            }
        })
        .collect();

    play_laydown(&mut players, &mut die, &mut placements);

    placements.reverse();
    GameResult { placements }
}

pub fn play_round(players: &[Player], starting_ind: usize, die: &mut Die) -> RoundResult {
    let mut loser_ind;
    let mut winner_ind;
    let mut loser_score;
    let mut winner_score;

    let starting_player = &players[starting_ind];
    let (starting_score, rules) = starting_player.play_start(die);
    if starting_score == u64::MAX {
        return RoundResult {
            winner: starting_ind,
            loser: starting_ind,
        };
    }
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
        if next_score == u64::MAX {
            return RoundResult {
                winner: next_ind,
                loser: next_ind,
            };
        }

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

pub fn roll_start(num_players: usize, die: &mut Die) -> usize {
    let mut player_results = vec![];
    for _ in 0..num_players {
        let player_result = die.roll();
        player_results.push(player_result);
    }
    let max_roll = player_results.iter().max().unwrap();
    let max_players: Vec<(usize, u8)> = player_results
        .iter()
        .enumerate()
        .filter_map(|(ind, res)| (res == max_roll).then_some((ind, *res)))
        .collect();
    if max_players.len() > 1 {
        let new_res = roll_start(max_players.len(), die);
        max_players.get(new_res).unwrap().0
    } else {
        max_players.first().unwrap().0
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

pub fn play_pickup(
    players: &mut Vec<Player>,
    num_tokens: u8,
    die: &mut Die,
    placements: &mut Vec<String>,
) {
    let mut start_ind = 0;

    let mut remaining_tokens = num_tokens;
    while remaining_tokens != 0 {
        println!("starting player: {}", players[start_ind].name);
        let result = play_round(&players, start_ind, die);
        if result.winner == result.loser {
            let winner = players.get(result.winner).unwrap();
            placements.push(winner.name.clone());
            players.remove(result.winner);
            start_ind = roll_start(players.len(), die);
            continue;
        }
        let loser = players.get_mut(result.loser).unwrap();
        loser.num_tokens += 1;
        println!(
            "loser is {}, now has {} tokens",
            loser.name, loser.num_tokens
        );
        start_ind = result.loser;
        remaining_tokens -= 1;
    }
}

pub fn play_laydown(players: &mut Vec<Player>, die: &mut Die, placements: &mut Vec<String>) {
    let num_tokens: u8 = players.iter().map(|pl| pl.num_tokens).sum();
    let mut start_ind;

    let mut remaining_tokens = num_tokens;
    while remaining_tokens != 0 {
        start_ind = next_start(&players, die);
        let result = play_round(&players, start_ind, die);

        let winner = players.get_mut(result.winner).unwrap();
        if result.winner == result.loser {
            remaining_tokens -= winner.num_tokens;
            winner.num_tokens = 0;
        } else {
            winner.num_tokens -= 1;
            remaining_tokens -= 1;
        }
        println!(
            "winner is {}, now has {} tokens",
            winner.name, winner.num_tokens
        );
        if winner.num_tokens == 0 {
            println!("{} finished the round", winner.name);
            placements.push(winner.name.clone());
            players.remove(result.winner);
        }
    }
}
