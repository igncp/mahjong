use crate::{Game, PlayerId};

impl Game {
    pub fn calculate_hand_score(&mut self, winner_player: &PlayerId) {
        let score = &mut self.score;
        let current_player_score = score.get(winner_player);
        if current_player_score.is_none() {
            return;
        }

        let current_player_score = current_player_score.unwrap();

        score.insert(winner_player.clone(), current_player_score + 1);
        //   const handMelds = getHandMelds({
        //     hand,
        //   });

        //   const roundWind = round.type;
        //   const seatWind = windsRoundsOrder[round.dealerPlayerIndex];

        //   const stats = Object.values(handMelds.melds).reduce(
        //     (acc, meld) => {
        //       const subHand = (meld || []).map((tile) => tile.id);
        //       const opts = {
        //         boardTilePlayerDiff: null,
        //         claimedTile: null,
        //         deck,
        //         subHand,
        //       };

        //       if (getIsPung(opts)) {
        //         acc.pungs++;
        //         if (deck[subHand[0]].type === TileType.Dragon) {
        //           acc.dragonPungs++;
        //         } else if (deck[subHand[0]].type === TileType.Wind) {
        //           if (deck[subHand[0]].value === roundWind) {
        //             acc.windPoints++;
        //           }
        //           if (deck[subHand[0]].value === seatWind) {
        //             acc.windPoints++;
        //           }
        //         }
        //       } else if (getIsChow(opts)) {
        //         acc.chows++;
        //       } else if (getIsKong(opts)) {
        //         acc.kongs++;
        //         if (deck[subHand[0]].type === TileType.Dragon) {
        //           acc.dragonKongs++;
        //         } else if (deck[subHand[0]].type === TileType.Wind) {
        //           if (deck[subHand[0]].value === roundWind) {
        //             acc.windPoints++;
        //           }
        //           if (deck[subHand[0]].value === seatWind) {
        //             acc.windPoints++;
        //           }
        //         }
        //       }

        //       return acc;
        //     },
        //     {
        //       pungs: 0,
        //       chows: 0,
        //       kongs: 0,
        //       dragonPungs: 0,
        //       dragonKongs: 0,
        //       windPoints: 0,
        //     }
        //   );

        //   let newPoints = 0;
        //   const tiles = hand.map((tile) => deck[tile.id]);

        //   if (stats.pungs === 4) newPoints += 6;
        //   if (stats.chows === 4) newPoints += 2;

        //   if (stats.dragonPungs === 2) newPoints += 6;
        //   else if (stats.dragonPungs) newPoints += stats.dragonPungs * 2;

        //   if (stats.dragonKongs) newPoints += stats.dragonKongs * 2;
        //   if (!round.tileClaimed) newPoints += 1;

        //   newPoints += tiles.filter((tile) =>
        //     [TileType.Season, TileType.Flower].includes(tile.type)
        //   ).length;

        //   newPoints += stats.windPoints * 2;

        //   score[winnerPlayer] = (score[winnerPlayer] || 0) + newPoints;
    }
}
