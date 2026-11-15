module Game.Meld (
    Meld (..),
    isPung,
    isChow,
    isKong,
    possibleMelds,
) where

import qualified Game.Tile as GameTile
import qualified Game.Types as GameTypes
import Data.List (sort)

data Meld = Meld
    { concealed :: Bool
    , meldTiles :: [GameTile.Tile]
    }
    deriving (Eq, Show)

isPung :: [GameTile.Tile] -> Bool
isPung [first, second, third] =
    not (isBonus first) && first == second && second == third
isPung _ = False

isChow :: Maybe Int -> Maybe GameTile.Tile -> [GameTile.Tile] -> Bool
isChow playerDiff claimedTile tiles =
    length tiles == 3
        && claimedTileCanBeUsed
        && case sortedSuitTiles of
            Just [first, second, third] -> first + 1 == second && second + 1 == third
            _ -> False
  where
    claimedTileCanBeUsed = case (playerDiff, claimedTile) of
        (Just 1, _) -> True
        (Just _, Just claimed) -> claimed `notElem` tiles
        _ -> True
    sortedSuitTiles = do
        suitTiles <- traverse suitValue tiles
        case suitTiles of
            [] -> Nothing
            (suit, _):rest
                | all ((== suit) . fst) rest -> Just $ sort $ map snd suitTiles
                | otherwise -> Nothing

isKong :: [GameTile.Tile] -> Bool
isKong [first, second, third, fourth] =
    not (isBonus first) && first == second && second == third && third == fourth
isKong _ = False

possibleMelds :: Maybe Int -> [GameTile.Tile] -> [[GameTile.Tile]]
possibleMelds playerDiff tiles =
    [ candidate
    | candidate <- combinations 3 tiles
    , isPung candidate || isChow playerDiff Nothing candidate
    ]

isBonus :: GameTile.Tile -> Bool
isBonus (GameTile.FlowerTile _) = True
isBonus (GameTile.SeasonTile _) = True
isBonus _ = False

suitValue :: GameTile.Tile -> Maybe (GameTypes.Suit, Int)
suitValue (GameTile.SuitTile value suit) = Just (suit, value)
suitValue _ = Nothing

combinations :: Int -> [a] -> [[a]]
combinations 0 _ = [[]]
combinations _ [] = []
combinations count (value : values) = map (value :) (combinations (count - 1) values) ++ combinations count values
