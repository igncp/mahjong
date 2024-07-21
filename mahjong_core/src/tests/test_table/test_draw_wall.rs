#[cfg(test)]
mod test {
    use std::sync::Mutex;

    use crate::{DrawWall, DrawWallPlace, Tile, TileId, Wind, WINDS_ROUND_ORDER};

    #[test]
    fn test_get_next() {
        let tiles: Vec<(TileId, DrawWallPlace)> = Tile::ids_from_summary("一筒,二筒,三筒")
            .iter()
            .map(|&id| (id, DrawWallPlace::Segment(Wind::East)))
            .collect();

        let draw_wall_ref = Mutex::new(DrawWall::new_full(tiles));
        let dw = || draw_wall_ref.lock().unwrap();

        let check_next = |expected: &str| {
            WINDS_ROUND_ORDER.iter().for_each(|wind| {
                assert_eq!(dw().summary_next(wind), expected);
            });
        };

        check_next("三筒");

        dw().pop_for_wind(&Wind::East);

        check_next("二筒");

        dw().pop_for_wind(&Wind::West);

        check_next("二筒");

        dw().pop_for_wind(&Wind::East);
        dw().pop_for_wind(&Wind::East);

        check_next("");
    }
}
