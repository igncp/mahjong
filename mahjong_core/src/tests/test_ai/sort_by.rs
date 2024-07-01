#[cfg(test)]
mod test {
    use crate::{ai::sort_by_is_mahjong, meld::PossibleMeld};
    use pretty_assertions::assert_eq;

    #[test]
    fn it_puts_melds_with_mahjong_at_the_beginning() {
        let default_meld = PossibleMeld {
            is_mahjong: false,
            player_id: "0".to_string(),
            tiles: vec![],
            discard_tile: None,
        };
        let mut melds = [
            PossibleMeld {
                player_id: "1".to_string(),
                ..default_meld.clone()
            },
            PossibleMeld {
                player_id: "2".to_string(),
                is_mahjong: true,
                ..default_meld.clone()
            },
            PossibleMeld {
                player_id: "3".to_string(),
                ..default_meld
            },
        ];

        melds.sort_by(sort_by_is_mahjong);
        let melds_ids = melds
            .iter()
            .map(|m| m.player_id.clone())
            .collect::<Vec<String>>();

        assert_eq!(
            melds_ids,
            vec!["2".to_string(), "1".to_string(), "3".to_string(),]
        );
    }
}
