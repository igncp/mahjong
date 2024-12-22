#[cfg(test)]
mod test {
    use crate::{
        ai::{PlayExitLocation, StandardAI},
        game::DrawError,
        Game,
    };
    use pretty_assertions::assert_eq;
    use rustc_hash::FxHashSet;
    use strum::IntoEnumIterator;

    #[test]
    fn play_action_results() {
        for exit_location in PlayExitLocation::iter() {
            let summary = match exit_location.clone() {
                PlayExitLocation::AutoStoppedDrawMahjong => {
                    "- P1: 三筒 七筒,八筒,九筒 一萬,二萬,三萬 四萬,五萬,六萬 七萬,八萬,九萬
                     Turn: P3, Phase: Playing
                     Discarded: 三筒"
                }
                PlayExitLocation::MeldCreated => {
                    "- P2: 一萬,一萬,一萬,三萬,五萬,七萬,九萬,一筒,三筒,五筒,七筒,九筒,一索,三索
                     Turn: P2, Phase: Playing"
                }
                PlayExitLocation::NewRoundFromMeld => "",
                PlayExitLocation::NoAutoDrawTile => "",
                PlayExitLocation::AutoStoppedDrawNormal => "",
                PlayExitLocation::NoAction => "",
                PlayExitLocation::TileDiscarded => "",
                PlayExitLocation::TurnPassed => "",
                PlayExitLocation::WaitingPlayers => {
                    "- P1: 一筒,三筒,八筒,九筒 一萬,二萬,三萬 四萬,五萬,六萬 七萬,八萬,九萬
                     - XP2
                     Turn: P1, Phase: Waiting Players"
                }
                PlayExitLocation::SuccessMahjong => {
                    "- P2: 三筒,三筒 七筒,八筒,九筒 一萬,二萬,三萬 四萬,五萬,六萬 七萬,八萬,九萬
                     Turn: P2, Phase: Playing
                     Drawn: 三筒"
                }
                PlayExitLocation::StartGame => "Phase: Beginning",
                PlayExitLocation::DecidedDealer => {
                    "Phase: Deciding Dealer, Initial Winds: 東,南,西,北"
                }
                PlayExitLocation::InitialDraw => {
                    "Wall: Random
                     Phase: Initial Draw"
                }
                PlayExitLocation::AlreadyEnd => "Phase: End",
                PlayExitLocation::ClaimedTile => {
                    "- P2: 一筒,三筒,八筒,九筒 一萬,二萬,三萬 四萬,五萬,六萬 七萬,八萬,九萬
                     Board: 二筒
                     Turn: P1, Phase: Playing
                     Discarded: 二筒"
                }
                // Invalid state, the board doesn't contain the discarded tile
                PlayExitLocation::CouldNotClaimTile => {
                    "- P2: 一筒,三筒,八筒,九筒 一萬,二萬,三萬 四萬,五萬,六萬 七萬,八萬,九萬
                     Turn: P1, Phase: Playing
                     Discarded: 二筒"
                }
                PlayExitLocation::RoundPassed => {
                    "- P1: 一筒,一筒,九筒,九筒 一萬,二萬,三萬 四萬,五萬,六萬 七萬,八萬,九萬
                     - P2: 一筒,一筒,九筒,九筒 一萬,二萬,三萬 四萬,五萬,六萬 七萬,八萬,九萬
                     - P3: 一筒,一筒,九筒,九筒 一萬,二萬,三萬 四萬,五萬,六萬 七萬,八萬,九萬
                     - P4: 一筒,一筒,九筒,九筒 一萬,二萬,三萬 四萬,五萬,六萬 七萬,八萬,九萬
                     Wall:
                     Turn: P1, Dealer: P1, Round: 1, Wind: 東, Phase: Playing"
                }
                PlayExitLocation::AIPlayerTurnPassed => {
                    "- P1: 一筒,一筒,九筒,九筒 一萬,二萬,三萬 四萬,五萬,六萬 七萬,八萬,九萬
                     - P2: 一筒,一筒,九筒,九筒 一萬,二萬,三萬 四萬,五萬,六萬 七萬,八萬,九萬
                     - P3: 一筒,一筒,九筒,九筒 一萬,二萬,三萬 四萬,五萬,六萬 七萬,八萬,九萬
                     - P4: 一筒,一筒,九筒,九筒 一萬,二萬,三萬 四萬,五萬,六萬 七萬,八萬,九萬
                     Turn: P2, Dealer: P1, Round: 1, Wind: 東, Phase: Playing
                     Discarded: 二筒, Drawn: 二筒"
                }
                PlayExitLocation::AIPlayerTileDrawn => {
                    "Wall: Random
                     Turn: P2, Dealer: P1, Round: 1, Wind: 東, Phase: Playing"
                }
                PlayExitLocation::TileDrawn => {
                    "Wall: Random
                     Turn: P1, Dealer: P1, Round: 1, Wind: 東, Phase: Playing"
                }
                PlayExitLocation::InitialShuffle => "Phase: Initial Shuffle",
                PlayExitLocation::InitialDrawError(e) => match e {
                    DrawError::NotEnoughTiles => "Phase: Initial Draw",
                },
                PlayExitLocation::WaitingDealerOrder => "Phase: Deciding Dealer",
                PlayExitLocation::CompletedPlayers => "Phase: Waiting Players",
                PlayExitLocation::FinishedCharleston => "",
            };

            if summary.is_empty() {
                continue;
            }

            let mut game = Game::from_summary(summary);
            let ai_players = FxHashSet::from_iter(
                game.players
                    .0
                    .clone()
                    .iter()
                    .enumerate()
                    .filter(|(idx2, _)| *idx2 != 0)
                    .map(|(_, p)| p.clone()),
            );
            let mut auto_stop_claim_meld = FxHashSet::default();
            auto_stop_claim_meld.insert("0".to_string());
            let mut game_ai = StandardAI::new(&mut game, ai_players, auto_stop_claim_meld);

            game_ai.can_draw_round = true;

            let actual = game_ai.play_action(false);

            assert_eq!(
                actual.exit_location, exit_location,
                "Test case: {:?}",
                exit_location
            );
        }
    }
}
