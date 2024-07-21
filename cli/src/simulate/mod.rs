use mahjong_core::{ai::StandardAI, Game, GamePhase};
use rustc_hash::FxHashSet;

pub use self::simulate_cli::{get_simulate_command, get_simulate_opts, SimulateOpts};
use self::stats::Stats;

mod simulate_cli;
mod stats;

pub async fn run_simulation(opts: SimulateOpts) {
    let mut stats = Stats::new();

    loop {
        let mut game = Game::new(None);
        let auto_stop_claim_meld = FxHashSet::default();
        let ai_players = FxHashSet::from_iter(game.players.0.clone());
        let mut game_ai = StandardAI::new(&mut game, ai_players, auto_stop_claim_meld);

        game_ai.dealer_order_deterministic = Some(false);
        game_ai.can_draw_round = true;

        loop {
            let result = game_ai.play_action();
            assert!(
                result.changed,
                "Didn't change anything in the round\n{}\n{:?}",
                game_ai.game.get_summary(),
                result
            );

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
