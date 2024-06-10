use chrono::Utc;
use mahjong_core::{Game, PlayerId};
use rustc_hash::FxHashMap;
use std::ops::{Add, Div};

fn average<T: Add<Output = T> + Div<Output = T> + Default + Copy + Into<f64>>(
    numbers: &Vec<T>,
) -> f64 {
    let mut total: T = Default::default();
    for numb in numbers {
        total = total + *numb;
    }
    total.into() / numbers.len() as f64
}

pub struct Stats {
    games_num: usize,
    games_per_second: Vec<(f32, usize)>,
    winners: FxHashMap<PlayerId, u32>,
    rounds_num: Vec<u32>,
    start_time: chrono::NaiveTime,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            games_num: 0,
            games_per_second: vec![],
            rounds_num: vec![],
            start_time: Utc::now().time(),
            winners: FxHashMap::default(),
        }
    }

    pub fn complete_game(&mut self, game: &Game) {
        // `max_by_key` favors the last value so need to randomize the keys
        let mut players = game.players.clone();

        players.shuffle();

        let winner = players.iter().max_by_key(|k| game.score.get(k).unwrap());

        if let Some(winner) = winner {
            *self.winners.entry(winner.clone()).or_insert(0) += 1;
        }

        self.games_num += 1;
        self.rounds_num.push(game.round.round_index + 1);
    }

    pub fn print_if_interval(&mut self, seconds: usize) {
        let end_time = Utc::now().time();
        let diff = (end_time - self.start_time).num_seconds() as usize;

        if diff > seconds {
            let last_average = self.games_per_second.last().unwrap_or(&(0.0, 0));
            let games_per_s = (self.games_num as f32 - last_average.1 as f32) / diff as f32;

            self.games_per_second.push((games_per_s, self.games_num));

            let gps = self
                .games_per_second
                .iter()
                .map(|(games_per_s, _)| *games_per_s)
                .collect::<Vec<f32>>();
            let average_games_per_s = average(&gps);
            let average_rounds_per_game = average(&self.rounds_num);

            self.start_time = end_time;

            println!("Winners: {:?}", self.winners);
            println!("Number of games: {:?}", self.games_num);
            println!("Average games per second: {:.2}", average_games_per_s);
            println!("Average rounds per game: {:.2}", average_rounds_per_game);
            println!("---\n");
        }
    }
}
