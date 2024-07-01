use crate::{game_summary::GameSummary, PlayerId, TileId};

use super::StandardAI;

impl<'a> StandardAI<'a> {
    // This can become complex if it takes into account different scoring rules.
    // For now it should only take into account the possibility to create a meld.
    // In future it should review which tiles have been claimed by other players.
    // Should have some unit tests.
    // TODO: finalise
    pub fn get_best_drops(&self, player_id: &PlayerId) -> Option<Vec<TileId>> {
        let game_clone = self.game.clone();
        let game_summary = GameSummary::from_game(&game_clone, player_id)?;

        if !game_summary.hand.can_drop_tile() {
            return None;
        }

        struct TileDrop {
            id: TileId,
            score: usize,
        }

        let mut drops: Vec<TileDrop> = vec![];

        for tile in game_summary.hand.list.iter() {
            if tile.set_id.is_some() {
                drops.push(TileDrop {
                    id: tile.id,
                    score: 0,
                });
            }

            // - Check how possible is it to build a meld with this tile (score can also be one if
            // one tile left)
        }

        // Best drops sorted from left to right
        drops.sort_by(|a, b| a.score.cmp(&b.score));

        let best_drops = drops.iter().map(|drop| drop.id).collect::<Vec<TileId>>();

        Some(best_drops)
    }
}
