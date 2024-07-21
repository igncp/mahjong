use std::time::SystemTime;

use mahjong_core::ai::{PlayActionResult, PlayExitLocation, StandardAI};
use service_contracts::{GameSettings, ServiceGame};

pub struct AIWrapper<'a> {
    standard_ai: StandardAI<'a>,
    game_settings: &'a mut GameSettings,
}

impl<'a> AIWrapper<'a> {
    pub fn new(service_game: &'a mut ServiceGame, draw_tile_for_real_player: Option<bool>) -> Self {
        let ai_players = service_game.get_ai_players();

        let mut standard_ai = StandardAI::new(
            &mut service_game.game,
            ai_players,
            service_game.settings.auto_stop_claim_meld.clone(),
        );

        if let Some(draw_tile_for_real_player) = draw_tile_for_real_player {
            standard_ai.draw_tile_for_real_player = draw_tile_for_real_player;
        }

        standard_ai.dealer_order_deterministic = Some(false);
        // This should be a setting in future
        standard_ai.with_dead_wall = false;

        Self {
            standard_ai,
            game_settings: &mut service_game.settings,
        }
    }

    pub fn play_action(&mut self) -> PlayActionResult {
        let discard_wait_ms = self.game_settings.discard_wait_ms;
        let last_discard_time = self.game_settings.last_discard_time;
        let now_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i128;

        self.standard_ai.can_pass_turn = discard_wait_ms.is_none()
            || last_discard_time == 0
            || (discard_wait_ms.unwrap() != -1 && {
                now_time <= last_discard_time
                    || (now_time - last_discard_time) >= discard_wait_ms.unwrap() as i128
            });

        let current_player = self.standard_ai.game.get_current_player();
        self.standard_ai.sort_on_draw = self
            .game_settings
            .auto_sort_players
            .contains(&current_player);

        let result = self.standard_ai.play_action();

        if result.changed && result.exit_location == PlayExitLocation::TileDiscarded {
            self.game_settings.last_discard_time = now_time;
        }

        result
    }

    pub fn get_is_after_discard(&self) -> bool {
        self.standard_ai.get_is_after_discard()
    }
}
