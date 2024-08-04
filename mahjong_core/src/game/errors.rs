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
    EndRound,
    NotMeld,
    TileIsPartOfMeld,
}

#[derive(Debug, PartialEq, Eq, Clone, EnumIter)]
pub enum PassNullRoundError {
    HandCanDropTile,
    HandCanSayMahjong,
    WallNotEmpty,
}

#[derive(Debug, PartialEq, Eq, Clone, EnumIter)]
pub enum BreakMeldError {
    MeldIsKong,
    MissingHand,
    TileIsExposed,
}

#[derive(Debug, PartialEq, Eq, Clone, EnumIter)]
pub enum DecideDealerError {
    DuplicatedWinds,
}

#[derive(Debug, PartialEq, Eq, Clone, EnumIter)]
pub enum DrawError {
    NotEnoughTiles,
}

impl Default for DrawError {
    fn default() -> Self {
        Self::NotEnoughTiles
    }
}
