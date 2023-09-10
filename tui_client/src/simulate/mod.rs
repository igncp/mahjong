use mahjong_core::{ai::StandardAI, Game, GamePhase};
use rustc_hash::FxHashSet;

pub use self::simulate_cli::get_simulate_command;
use self::stats::Stats;

mod simulate_cli;
mod stats;

pub async fn run_simulation() {
    let mut stats = Stats::new();

    loop {
        let mut game = Game::default();
        game.start_game();

        let ai_players = FxHashSet::from_iter(game.players.clone());
        let auto_stop_claim_meld = FxHashSet::default();
        let mut game_ai = StandardAI::new(&mut game, ai_players, auto_stop_claim_meld);

        game_ai.can_draw_round = true;

        loop {
            let result = game_ai.play_action();
            assert!(result.changed, "Didn't change anything in the round");

            if game_ai.game.phase != GamePhase::Playing {
                stats.complete_game(game_ai.game);
                break;
            }
        }

        stats.print_if_interval(10);
    }
}
