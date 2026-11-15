module Game (
    Command (CommandClaimTile, CommandDrawTile, CommandInitialDraw, CommandPrepareTable),
    CommandClaimTile (..),
    ClaimTileError (..),
    allClaimTileErrors,
    CommandDiscardTile (..),
    CommandDrawTile (..),
    CommandInitialDraw (..),
    CommandPrepareTable (..),
    DrawError (..),
    allDrawErrors,
    DrawTileResult (..),
    GameState,
    PlayerHand (..),
    initialGameState,
    id,
    name,
    phase,
    players,
    style,
    playerHands,
    wall,
    board,
    turn,
    dealer,
    round,
    roundWind,
    consecutive,
    discarded,
    drawn,
    DiscardTileError (..),
    allDiscardTileErrors,
    discardTile,
    commandDrawTile,
    commandClaimTile,
    commandDiscardTile,
    commandInitialDraw,
    commandPrepareTable,
    runCommand,
) where

import Data.Either (fromRight)
import Data.List (find)
import qualified Game.Meld as GameMeld
import qualified Game.Tile as GameTile
import qualified Game.Types as GameTypes
import Prelude hiding (id, round)

data Command
    = CommandDrawTile CommandDrawTile
    | CommandClaimTile CommandClaimTile
    | CommandDiscardTile CommandDiscardTile
    | CommandInitialDraw CommandInitialDraw
    | CommandPrepareTable CommandPrepareTable
    deriving (Eq, Show)

data CommandDrawTile = CommandDrawTilePayload
    deriving (Eq, Show)

newtype CommandClaimTile = CommandClaimTilePayload Int
    deriving (Eq, Show)

data ClaimTileError
    = MissingHand
    | HandHasWrongTileCount
    | NoClaimedTile
    | BoardEmpty
    deriving (Eq, Show, Enum, Bounded)

allClaimTileErrors :: [ClaimTileError]
allClaimTileErrors = [minBound .. maxBound]

newtype CommandDiscardTile = CommandDiscardTilePayload GameTile.Tile
    deriving (Eq, Show)

data CommandInitialDraw = CommandInitialDrawPayload
    deriving (Eq, Show)

data CommandPrepareTable = CommandPrepareTablePayload
    { withDeadWall :: Bool
    , shuffleTiles :: Bool
    }
    deriving (Eq, Show)

data DrawError = NotEnoughTiles
    deriving (Eq, Show, Enum, Bounded)

allDrawErrors :: [DrawError]
allDrawErrors = [minBound .. maxBound]

data DrawTileResult
    = AlreadyDrawn
    | DrawTileNormal GameTile.Tile
    | WallExhausted
    deriving (Eq, Show)

data GameState = GameState
    { id :: String
    , name :: String
    , phase :: GameTypes.Phase
    , players :: [GameTypes.PlayerId]
    , style :: GameTypes.GameStyle
    , playerHands :: [(Int, PlayerHand)]
    , wall :: Maybe [GameTile.Tile]
    , board :: [GameTile.Tile]
    , turn :: Int
    , dealer :: Int
    , round :: Int
    , roundWind :: GameTypes.Wind
    , consecutive :: Int
    , discarded :: Maybe (GameTile.Tile, Maybe Int)
    , drawn :: Maybe GameTile.Tile
    }
    deriving (Eq, Show)

data DiscardTileError
    = ClaimedAnotherTile
    | NoPlayerCanDiscard
    | PlayerHasNoTile
    | TileIsExposed
    | TileIsPartOfMeld
    deriving (Eq, Show, Enum, Bounded)

allDiscardTileErrors :: [DiscardTileError]
allDiscardTileErrors = [minBound .. maxBound]

newtype PlayerHand = PlayerHand {melds :: [GameMeld.Meld]}
    deriving (Eq, Show)

initialGameState :: GameState
initialGameState =
    GameState
        { id = ""
        , name = ""
        , phase = GameTypes.Beginning
        , players = []
        , style = GameTypes.HongKong
        , playerHands = []
        , wall = Nothing
        , board = []
        , turn = 1
        , dealer = 1
        , round = 1
        , roundWind = GameTypes.East
        , consecutive = 0
        , discarded = Nothing
        , drawn = Nothing
        }

discardTile :: GameTile.Tile -> GameState -> Either DiscardTileError GameState
discardTile tile game = do
    (player, hand) <- maybe (Left NoPlayerCanDiscard) Right $ find ((== 14) . handSize . snd) (playerHands game)
    (meldIndex, tileIndex, meld) <- maybe (Left PlayerHasNoTile) Right $ findDiscardableTile tile (melds hand)
    if not (GameMeld.concealed meld)
        then Left TileIsExposed
        else
            if meldIndex > 0
                then Left TileIsPartOfMeld
                else case discarded game of
                    Just (claimedTile, Just claimedBy)
                        | claimedBy == player && claimedTile /= tile -> Left ClaimedAnotherTile
                    _ ->
                        Right
                            game
                                { playerHands = updateHand player (removeTile meldIndex tileIndex hand) (playerHands game)
                                , board = board game ++ [tile]
                                , discarded = Just (tile, Nothing)
                                }
  where
    handSize = sum . map (length . GameMeld.meldTiles) . melds

findDiscardableTile :: GameTile.Tile -> [GameMeld.Meld] -> Maybe (Int, Int, GameMeld.Meld)
findDiscardableTile tile handMelds = case matches of
    [] -> Nothing
    [match] -> Just match
    _ -> find (\(meldIndex, _, _) -> meldIndex == 0) matches
  where
    matches =
        [ (meldIndex, tileIndex, meld)
        | (meldIndex, meld) <- zip [0 ..] handMelds
        , (tileIndex, candidate) <- zip [0 ..] (GameMeld.meldTiles meld)
        , candidate == tile
        ]

removeTile :: Int -> Int -> PlayerHand -> PlayerHand
removeTile meldIndex tileIndex hand = PlayerHand $ zipWith update [0 ..] (melds hand)
  where
    removeFromMeld meld = meld{GameMeld.meldTiles = removeAt tileIndex (GameMeld.meldTiles meld)}
    update index meld
        | index == meldIndex = removeFromMeld meld
        | otherwise = meld

updateHand :: Int -> PlayerHand -> [(Int, PlayerHand)] -> [(Int, PlayerHand)]
updateHand player updatedHand = map replace
  where
    replace (currentPlayer, hand)
        | currentPlayer == player = (currentPlayer, updatedHand)
        | otherwise = (currentPlayer, hand)

removeAt :: Int -> [a] -> [a]
removeAt index values = take index values ++ drop (index + 1) values

commandPrepareTable :: GameState -> CommandPrepareTable -> GameState
commandPrepareTable state _ =
    state
        { wall = Just [GameTile.FlowerTile GameTypes.Plum]
        , phase = GameTypes.InitialDraw
        }

commandDrawTile :: GameState -> CommandDrawTile -> (GameState, DrawTileResult)
commandDrawTile state _
    | Just _ <- drawn state = (state, AlreadyDrawn)
    | Nothing <- wall state = (state, WallExhausted)
    | Just [] <- wall state = (state, WallExhausted)
    | Just (tile : remainingWall) <- wall state =
        ( state
            { wall = Just remainingWall
            , drawn = Just tile
            }
        , DrawTileNormal tile
        )

commandClaimTile :: GameState -> CommandClaimTile -> Either ClaimTileError GameState
commandClaimTile state (CommandClaimTilePayload player) = do
    hand <- maybe (Left MissingHand) Right $ lookup player (playerHands state)
    if handSize hand /= 13
        then Left HandHasWrongTileCount
        else case discarded state of
            Nothing -> Left NoClaimedTile
            Just _ -> case board state of
                [] -> Left BoardEmpty
                claimedTile : remainingBoard ->
                    Right $
                        state
                            { playerHands = updateHand player (addTile claimedTile hand) (playerHands state)
                            , board = remainingBoard
                            , turn = player
                            , discarded = Just (claimedTile, Just player)
                            }
  where
    handSize = sum . map (length . GameMeld.meldTiles) . melds
    addTile tile (PlayerHand []) = PlayerHand [GameMeld.Meld True [tile]]
    addTile tile (PlayerHand (meld : rest)) = PlayerHand (meld{GameMeld.meldTiles = GameMeld.meldTiles meld ++ [tile]} : rest)

commandDiscardTile :: GameState -> CommandDiscardTile -> Either DiscardTileError GameState
commandDiscardTile state (CommandDiscardTilePayload tile) = discardTile tile state

commandInitialDraw :: GameState -> CommandInitialDraw -> Either DrawError GameState
commandInitialDraw state _ = foldl drawForPlayer (Right state) (playerHands state)
  where
    drawForPlayer drawnState (player, hand) = do
        game <- drawnState
        let tileCount = sum $ map (length . GameMeld.meldTiles) (melds hand)
        if tileCount >= 13
            then Right game
            else Left NotEnoughTiles

runCommand :: GameState -> Command -> GameState
runCommand state command = case command of
    CommandDrawTile payload -> fst $ commandDrawTile state payload
    CommandClaimTile payload -> fromRight state $ commandClaimTile state payload
    CommandDiscardTile payload -> fromRight state $ commandDiscardTile state payload
    CommandInitialDraw payload -> fromRight state $ commandInitialDraw state payload
    CommandPrepareTable payload -> commandPrepareTable state payload
