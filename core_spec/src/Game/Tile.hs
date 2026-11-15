module Game.Tile (Tile (..)) where

import qualified Game.Types as GameTypes

data Tile
    = SuitTile Int GameTypes.Suit
    | WindTile GameTypes.Wind
    | DragonTile GameTypes.Dragon
    | FlowerTile GameTypes.Flower
    | SeasonTile GameTypes.Season
    deriving (Eq, Show)
