macro_rules! derive_game_common {
    ($i:item) => {
        #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
        $i
    };
}

pub(crate) use derive_game_common;
