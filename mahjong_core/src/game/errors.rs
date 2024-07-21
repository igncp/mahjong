use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Eq, Clone, EnumIter)]
pub enum DiscardTileError {
    ClaimedAnotherTile,
    NoPlayerCanDiscard,
    PlayerHasNoTile,
    TileIsExposed,
    TileIsPartOfMeld,
}

#[derive(Debug, PartialEq, Eq, Clone, EnumIter)]
pub enum CreateMeldError {
    NotMeld,
    TileIsPartOfMeld,
}

#[derive(Debug, PartialEq, Eq, Clone, EnumIter)]
pub enum PassNullRoundError {
    HandCanDropTile,
    HandCanSayMahjong,
}

#[derive(Debug, PartialEq, Eq, Clone, EnumIter)]
pub enum BreakMeldError {
    MissingHand,
    TileIsExposed,
}

#[derive(Debug, PartialEq, Eq, Clone, EnumIter)]
pub enum DecideDealerError {
    DuplicatedWinds,
}

#[derive(Debug, PartialEq, Eq, Clone, EnumIter)]
pub enum InitialDrawError {
    NotEnoughTiles,
}

impl Default for InitialDrawError {
    fn default() -> Self {
        Self::NotEnoughTiles
    }
}
