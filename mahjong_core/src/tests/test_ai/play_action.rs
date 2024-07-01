#[cfg(test)]
mod test {
    use crate::{
        ai::{PlayExitLocation, StandardAI},
        Game,
    };
    use pretty_assertions::assert_eq;
    use rustc_hash::FxHashSet;

    const PLAY_ACTION_FIXTURES: &[(&str, PlayExitLocation)] = &[
        (
            "- P2: 一筒,三筒,八筒,九筒 一萬,二萬,三萬 四萬,五萬,六萬 七萬,八萬,九萬
             Board: 二筒
             Turn: P1
             Discarded: 二筒",
            PlayExitLocation::ClaimedTile,
        ),
        (
            "Wall: Random
             Turn: P1, Dealer: P1, Round: 1, Wind: East, Phase: Playing",
            PlayExitLocation::TileDrawn,
        ),
        (
            "Wall: Random
             Turn: P2, Dealer: P1, Round: 1, Wind: East, Phase: Playing",
            PlayExitLocation::AIPlayerTileDrawn,
        ),
        (
            "- P1: 一筒,一筒,九筒,九筒 一萬,二萬,三萬 四萬,五萬,六萬 七萬,八萬,九萬
             - P2: 一筒,一筒,九筒,九筒 一萬,二萬,三萬 四萬,五萬,六萬 七萬,八萬,九萬
             - P3: 一筒,一筒,九筒,九筒 一萬,二萬,三萬 四萬,五萬,六萬 七萬,八萬,九萬
             - P4: 一筒,一筒,九筒,九筒 一萬,二萬,三萬 四萬,五萬,六萬 七萬,八萬,九萬
             Turn: P2, Dealer: P1, Round: 1, Wind: East, Phase: Playing
             Discarded: 二筒, Drawn: 二筒",
            PlayExitLocation::AIPlayerTurnPassed,
        ),
        // Invalid state, the board doesn't contain the discarded tile
        (
            "- P2: 一筒,三筒,八筒,九筒 一萬,二萬,三萬 四萬,五萬,六萬 七萬,八萬,九萬
             Turn: P1
             Discarded: 二筒",
            PlayExitLocation::CouldNotClaimTile,
        ),
    ];

    #[test]
    fn play_action_results() {
        for (idx, (input, expected)) in PLAY_ACTION_FIXTURES.iter().enumerate() {
            let mut game = Game::from_summary(input);
            let ai_players = FxHashSet::from_iter(
                game.players
                    .0
                    .clone()
                    .iter()
                    .enumerate()
                    .filter(|(idx2, _)| *idx2 != 0)
                    .map(|(_, p)| p.clone()),
            );
            let auto_stop_claim_meld = FxHashSet::default();
            let mut game_ai = StandardAI::new(&mut game, ai_players, auto_stop_claim_meld);

            let actual = game_ai.play_action();

            assert_eq!(actual.exit_location, *expected, "Test case: {}", idx);
        }
    }
}
