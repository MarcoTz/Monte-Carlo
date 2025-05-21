pub mod die;
pub mod player;
pub mod report;
pub mod results;
pub mod scoring;

use die::Die;
use game::Game;
use player::Player;
use results::{GameResult, RoundEnd, RoundResult, RoundType};
use scoring::ScoringMethod;
use std::collections::HashMap;

pub struct Chicago {
    players: Vec<Player>,
    die: Die,
    remaining_tokens: u64,
    results: GameResult,
}

pub struct GameConfig {
    num_players: u64,
}

impl GameConfig {
    pub fn new(num_players: u64) -> GameConfig {
        GameConfig { num_players }
    }
}

impl Game for Chicago {
    type Params = GameConfig;
    type Results = GameResult;

    fn init(params: &GameConfig) -> Self {
        Chicago {
            remaining_tokens: params.num_players + 1,
            players: (0..params.num_players)
                .map(|i| Player::new(&format!("player_{i}")))
                .collect(),
            die: Die::new(),
            results: GameResult::new(),
        }
    }

    fn run(mut self) -> Self::Results {
        self.play_game();
        self.results
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RoundRules {
    method: ScoringMethod,
    num_rolls: u8,
}

impl Chicago {
    fn remove_done(&mut self) {
        let mut to_remove = vec![];
        for (ind, pl) in self.players.iter().enumerate() {
            if pl.num_tokens == 0 {
                to_remove.push(ind)
            }
        }
        to_remove.reverse();
        for ind in to_remove {
            self.players.remove(ind);
        }
    }

    fn get_player_by_name(&mut self, name: &str) -> (usize, &mut Player) {
        self.players
            .iter_mut()
            .enumerate()
            .find(|(_, pl)| pl.name == name)
            .unwrap()
    }

    fn remove_player_by_name(&mut self, name: &str) {
        let ind = self
            .players
            .iter()
            .enumerate()
            .find(|(_, pl)| pl.name == name)
            .unwrap()
            .0;
        self.players.remove(ind);
    }

    pub fn play_game(&mut self) {
        self.play_pickup();
        self.remove_done();
        self.play_laydown();
        self.results.placements.reverse();
    }

    pub fn play_round(&mut self, starting_ind: usize, round_type: RoundType) -> RoundResult {
        let mut loser_ind;
        let mut winner_ind;
        let mut loser_score;
        let mut winner_score;
        let mut player_results = HashMap::new();

        let starting_player = &self.players[starting_ind];
        let (starting_score, rules) = starting_player.play_start(&mut self.die);
        player_results.insert(starting_player.name.clone(), starting_score);
        if starting_score == u64::MAX {
            return RoundResult {
                round_type,
                winner: starting_player.name.clone(),
                starting_player: starting_player.name.clone(),
                player_results,
                round_end: RoundEnd::Chic,
                rules,
            };
        }
        loser_ind = starting_ind;
        winner_ind = starting_ind;
        loser_score = starting_score;
        winner_score = starting_score;

        for i in 1..=(self.players.len() - 1) {
            let next_ind = (starting_ind + i) % self.players.len();
            let next_player = &self.players[next_ind];
            let next_score = next_player.play(&mut self.die, &rules, winner_score);
            player_results.insert(next_player.name.clone(), next_score);
            if next_score == u64::MAX {
                return RoundResult {
                    round_type,
                    winner: next_player.name.clone(),
                    starting_player: starting_player.name.clone(),
                    player_results,
                    round_end: RoundEnd::Chic,
                    rules,
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
        }
        RoundResult {
            round_type,
            round_end: RoundEnd::Normal {
                loser: self.players[loser_ind].name.clone(),
            },
            winner: self.players[winner_ind].name.clone(),
            player_results,
            starting_player: starting_player.name.clone(),
            rules,
        }
    }

    pub fn roll_start(&mut self, num_players: usize) -> usize {
        let mut player_results = vec![];
        for _ in 0..num_players {
            let player_result = self.die.roll();
            player_results.push(player_result);
        }
        let max_roll = player_results.iter().max().unwrap();
        let max_players: Vec<(usize, u8)> = player_results
            .iter()
            .enumerate()
            .filter_map(|(ind, res)| (res == max_roll).then_some((ind, *res)))
            .collect();
        if max_players.len() > 1 {
            let new_res = self.roll_start(max_players.len());
            max_players.get(new_res).unwrap().0
        } else {
            max_players.first().unwrap().0
        }
    }

    pub fn next_start(&mut self) -> usize {
        let max_tokens = self.players.iter().map(|pl| pl.num_tokens).max().unwrap();
        let mut candidates: Vec<usize> = (0..self.players.len())
            .filter(|i| self.players[*i].num_tokens == max_tokens)
            .collect();
        while candidates.len() > 1 {
            let results: Vec<(usize, u8)> = candidates
                .iter()
                .map(|ind| (*ind, self.die.roll()))
                .collect();
            let max_res = results
                .iter()
                .max_by(|(_, res1), (_, res2)| res1.cmp(res2))
                .unwrap()
                .1;
            candidates = results
                .into_iter()
                .filter_map(|(ind, res)| (res == max_res).then_some(ind))
                .collect();
        }
        candidates[0]
    }

    pub fn play_pickup(&mut self) {
        let mut start_ind = 0;

        while self.remaining_tokens != 0 {
            let result = self.play_round(start_ind, RoundType::Pickup);
            match result.round_end {
                RoundEnd::Chic => {
                    self.results.placements.push(result.winner.clone());
                    self.remove_player_by_name(&result.winner);
                    if self.players.len() == 0 {
                        return;
                    }
                    start_ind = self.roll_start(self.players.len());
                }
                RoundEnd::Normal { ref loser } => {
                    let (loser_ind, loser) = self.get_player_by_name(loser);
                    loser.num_tokens += 1;
                    start_ind = loser_ind;
                    self.remaining_tokens -= 1;
                }
            }
            self.results.round_results.push(result);
        }
    }

    pub fn play_laydown(&mut self) {
        let num_tokens: u8 = self.players.iter().map(|pl| pl.num_tokens).sum();
        let mut start_ind;

        let mut remaining_tokens = num_tokens;
        while remaining_tokens != 0 {
            start_ind = self.next_start();
            let result = self.play_round(start_ind, RoundType::Laydown);

            match result.round_end {
                RoundEnd::Chic => {
                    let (winner_ind, winner) = self.get_player_by_name(&result.winner);
                    remaining_tokens -= winner.num_tokens;
                    winner.num_tokens = 0;
                    self.results.placements.push(result.winner.clone());
                    self.players.remove(winner_ind);
                }
                RoundEnd::Normal { loser: _ } => {
                    let (winner_ind, winner) = self.get_player_by_name(&result.winner);
                    winner.num_tokens -= 1;
                    remaining_tokens -= 1;

                    if winner.num_tokens == 0 {
                        self.results.placements.push(result.winner.clone());
                        self.players.remove(winner_ind);
                    }
                }
            }
            self.results.round_results.push(result);
        }
    }
}
