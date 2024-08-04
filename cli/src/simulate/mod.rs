use std::process;

use mahjong_core::{
    ai::{PlayActionResult, StandardAI},
    Game, GamePhase,
};
use rustc_hash::FxHashSet;

pub use self::simulate_cli::{get_simulate_command, get_simulate_opts, SimulateOpts};
use self::stats::Stats;

mod simulate_cli;
mod stats;

#[derive(Debug)]
struct HistoryItem {
    game: Game,
    result: PlayActionResult,
}

pub async fn run_simulation(opts: SimulateOpts) {
    let mut stats = Stats::new();

    loop {
        let mut game = Game::new(None);
        let mut history: Option<Vec<HistoryItem>> =
            if opts.debug { Some(Vec::new()) } else { None };

        for player in 0..Game::get_players_num(&game.style) {
            game.players.push(player.to_string());
        }

        let auto_stop_claim_meld = FxHashSet::default();
        let ai_players = FxHashSet::from_iter(game.players.0.clone());
        let mut game_ai = StandardAI::new(&mut game, ai_players, auto_stop_claim_meld);

        game_ai.dealer_order_deterministic = Some(false);
        game_ai.can_draw_round = true;

        loop {
            let result = game_ai.play_action(opts.debug);

            if opts.debug {
                let history_vect = history.as_mut().unwrap();
                history_vect.push(HistoryItem {
                    game: game_ai.game.clone(),
                    result: result.clone(),
                });
            }

            if !result.changed {
                println!("Game didn't change, breaking");
                if opts.debug {
                    let history = history
                        .as_ref()
                        .unwrap()
                        .iter()
                        .rev()
                        .take(10)
                        .rev()
                        .collect::<Vec<_>>();

                    println!("History:");
                    for (idx, history_item) in history.iter().enumerate() {
                        if idx > 0 {
                            println!("---");

                            if idx == history.len() - 1 {
                                break;
                            }
                        }
                        println!("- {:?}", history_item.result);
                        println!("{}", history_item.game.get_summary());
                        println!("\n\n\n");
                    }
                }
                println!(
                    "Current state:\n{}\n{:?}",
                    game_ai.game.get_summary(),
                    result
                );
                process::exit(1);
            }

            if game_ai.game.phase == GamePhase::End {
                stats.complete_game(game_ai.game);
                break;
            }
        }

        let passed_interval = stats.print_if_interval(10);

        if passed_interval && opts.once {
            break;
        };
    }
}
