module Game.Types (
    Suit (..),
    Wind (..),
    Dragon (..),
    Flower (..),
    Phase (..),
    Season (..),
    PlayerId,
    GameStyle (..),
) where

data Suit = Bamboo | Dots | Characters
    deriving (Eq, Show)

data Wind = East | South | West | North
    deriving (Eq, Show)

data Dragon = Red | Green | White
    deriving (Eq, Show)

data Flower = Plum | Orchid | Chrysanthemum | FlowerBamboo
    deriving (Eq, Show)

data Phase
    = Beginning
    | DecidingDealer
    | End
    | InitialDraw
    | InitialShuffle
    | Playing
    | WaitingPlayers
    deriving (Eq, Show)

data Season = Spring | Summer | Autumn | Winter
    deriving (Eq, Show)

type PlayerId = String

data GameStyle = HongKong
    deriving (Eq, Show)
