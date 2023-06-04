#[cfg(test)]
mod test {
    use crate::{Hand, HandTile};

    #[test]
    fn test_sort_by_tiles() {
        let mut hand = Hand(vec![
            HandTile {
                concealed: true,
                id: 114,
                set_id: None,
            },
            HandTile {
                concealed: true,
                id: 15,
                set_id: None,
            },
            HandTile {
                concealed: true,
                id: 68,
                set_id: None,
            },
            HandTile {
                concealed: true,
                id: 16,
                set_id: None,
            },
            HandTile {
                concealed: true,
                id: 27,
                set_id: None,
            },
            HandTile {
                concealed: true,
                id: 121,
                set_id: Some("17808af0-bac7-4b03-946a-2b50f0ffa02b".to_string()),
            },
            HandTile {
                concealed: true,
                id: 122,
                set_id: Some("17808af0-bac7-4b03-946a-2b50f0ffa02b".to_string()),
            },
            HandTile {
                concealed: true,
                id: 55,
                set_id: Some("17808af0-bac7-4b03-946a-2b50f0ffa02b".to_string()),
            },
            HandTile {
                concealed: true,
                id: 74,
                set_id: None,
            },
            HandTile {
                concealed: true,
                id: 1,
                set_id: None,
            },
            HandTile {
                concealed: true,
                id: 23,
                set_id: None,
            },
            HandTile {
                concealed: true,
                id: 73,
                set_id: None,
            },
            HandTile {
                concealed: true,
                id: 6,
                set_id: None,
            },
        ]);

        hand.sort_by_tiles(&[74, 114, 15, 68, 16, 27, 1, 23, 73, 6]);

        let result_ids = hand.0.iter().map(|t| t.id).collect::<Vec<_>>();

        assert_eq!(
            result_ids,
            vec![74, 114, 15, 68, 16, 27, 1, 23, 73, 6, 121, 122, 55]
        );
    }
}
