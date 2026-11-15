module SummaryView (
    renderTile,
    parseTile,
    renderTiles,
    parseTiles,
    renderHand,
    parseHand,
    renderSummary,
    parseSummary,
) where

import Data.List (intercalate, stripPrefix)
import Game (
    GameState (..),
    PlayerHand (..),
    initialGameState,
 )
import qualified Game.Meld as GameMeld
import qualified Game.Tile as GameTile
import qualified Game.Types as GameTypes
import Text.Read (readMaybe)
import Utils (splitOn, stripSuffix)
import Prelude hiding (round)

renderTile :: GameTile.Tile -> String
renderTile (GameTile.SuitTile value suit) =
    chineseNumber value ++ case suit of
        GameTypes.Bamboo -> "索"
        GameTypes.Dots -> "筒"
        GameTypes.Characters -> "萬"
renderTile (GameTile.WindTile wind) = renderWind wind
renderTile (GameTile.DragonTile dragon) = case dragon of
    GameTypes.Red -> "中"
    GameTypes.Green -> "發"
    GameTypes.White -> "白"
renderTile (GameTile.FlowerTile flower) = case flower of
    GameTypes.Plum -> "梅"
    GameTypes.Orchid -> "蘭"
    GameTypes.Chrysanthemum -> "菊"
    GameTypes.FlowerBamboo -> "竹"
renderTile (GameTile.SeasonTile season) = case season of
    GameTypes.Spring -> "春"
    GameTypes.Summer -> "夏"
    GameTypes.Autumn -> "秋"
    GameTypes.Winter -> "冬"

parseTile :: String -> Either String GameTile.Tile
parseTile [number, suit] = do
    value <- maybe (Left $ "Invalid tile value: " ++ [number]) Right (lookup number chineseNumbers)
    parsedSuit <- case suit of
        '索' -> Right GameTypes.Bamboo
        '筒' -> Right GameTypes.Dots
        '萬' -> Right GameTypes.Characters
        _ -> Left $ "Invalid suit: " ++ [suit]
    Right $ GameTile.SuitTile value parsedSuit
parseTile [tile] = case tile of
    '東' -> Right $ GameTile.WindTile GameTypes.East
    '南' -> Right $ GameTile.WindTile GameTypes.South
    '西' -> Right $ GameTile.WindTile GameTypes.West
    '北' -> Right $ GameTile.WindTile GameTypes.North
    '中' -> Right $ GameTile.DragonTile GameTypes.Red
    '發' -> Right $ GameTile.DragonTile GameTypes.Green
    '白' -> Right $ GameTile.DragonTile GameTypes.White
    '梅' -> Right $ GameTile.FlowerTile GameTypes.Plum
    '蘭' -> Right $ GameTile.FlowerTile GameTypes.Orchid
    '菊' -> Right $ GameTile.FlowerTile GameTypes.Chrysanthemum
    '竹' -> Right $ GameTile.FlowerTile GameTypes.FlowerBamboo
    '春' -> Right $ GameTile.SeasonTile GameTypes.Spring
    '夏' -> Right $ GameTile.SeasonTile GameTypes.Summer
    '秋' -> Right $ GameTile.SeasonTile GameTypes.Autumn
    '冬' -> Right $ GameTile.SeasonTile GameTypes.Winter
    _ -> Left $ "Invalid tile: " ++ [tile]
parseTile tile = Left $ "Invalid tile: " ++ tile

renderTiles :: [GameTile.Tile] -> String
renderTiles = intercalate "," . map renderTile

parseTiles :: String -> Either String [GameTile.Tile]
parseTiles summary = traverse parseTile $ filter (not . null) $ splitOn ',' summary

renderHand :: PlayerHand -> String
renderHand (PlayerHand handMelds) = unwords $ map renderMeld handMelds
  where
    renderMeld meld = (if GameMeld.concealed meld then "" else "*") ++ renderTiles (GameMeld.meldTiles meld)

parseHand :: String -> Either String PlayerHand
parseHand "_" = Right $ PlayerHand []
parseHand summary = PlayerHand <$> traverse parseMeld (words summary)
  where
    parseMeld ('*' : tiles) = GameMeld.Meld False <$> parseTiles tiles
    parseMeld tiles = GameMeld.Meld True <$> parseTiles tiles

renderSummary :: GameState -> String
renderSummary game = intercalate "\n" $ playerLines ++ wallLine ++ boardLine ++ [roundLine, detailLine]
  where
    playerLines = ["- P" ++ show player ++ ": " ++ renderHand hand | (player, hand) <- playerHands game, not $ null (melds hand)]
    wallLine = case wall game of
        Nothing -> []
        Just [] -> []
        Just tiles -> ["Wall: " ++ (if length tiles > 1 then "... " else "") ++ renderTiles (take 1 tiles)]
    -- The Rust view shows most recent discards first.
    boardLine = case board game of
        [] -> []
        tiles -> ["Board: " ++ renderTiles (take 2 $ reverse tiles) ++ if length tiles > 2 then "..." else ""]
    roundLine =
        intercalate
            ", "
            [ "Turn: P" ++ show (turn game)
            , "Dealer: P" ++ show (dealer game)
            , "Round: " ++ show (round game)
            , "Wind: " ++ renderWind (roundWind game)
            , "Phase: " ++ renderPhase (phase game)
            ]
    detailLine = intercalate ", " $ ["Consecutive: " ++ show (consecutive game)] ++ discardedPart ++ drawnPart
    discardedPart = case discarded game of
        Nothing -> []
        Just (tile, claimedBy) -> ["Discarded: " ++ renderTile tile ++ maybe "" (("(P" ++) . (++ ")") . show) claimedBy]
    drawnPart = maybe [] (pure . ("Drawn: " ++) . renderTile) (drawn game)

parseSummary :: String -> Either String GameState
parseSummary input = foldl addLine (Right initialGameState) $ filter (not . null) $ map trim $ lines input
  where
    addLine parsed line = parsed >>= flip parseLine line

parseLine :: GameState -> String -> Either String GameState
parseLine summary line
    | Just rest <- stripPrefix "- P" line = do
        (player, hand) <- parsePlayer rest
        Right summary{playerHands = playerHands summary ++ [(player, hand)]}
    | Just rest <- stripPrefix "Wall: " line = do
        tiles <- parseTiles $ removeEllipsis rest
        Right summary{wall = Just tiles}
    | Just rest <- stripPrefix "Board: " line = do
        tiles <- parseTiles $ removeEllipsis rest
        -- Summary board tiles are displayed newest first; internal board is chronological.
        Right summary{board = reverse tiles}
    | Just rest <- stripPrefix "Turn: " line = parseRoundLine summary rest
    | Just rest <- stripPrefix "Consecutive: " line = parseDetailLine summary rest
    | otherwise = Left $ "Invalid summary line: " ++ line

parsePlayer :: String -> Either String (Int, PlayerHand)
parsePlayer input = case break (== ':') input of
    (player, ':' : ' ' : hand) -> do
        number <- parseNatural player
        parsedHand <- parseHand hand
        Right (number, parsedHand)
    _ -> Left $ "Invalid player line: - P" ++ input

parseRoundLine :: GameState -> String -> Either String GameState
parseRoundLine summary input = do
    fragments <- traverse splitFragment $ splitOn ',' ("Turn: " ++ input)
    turnNumber <- lookupRequired "Turn" fragments >>= parsePlayerNumber
    dealerNumber <- lookupRequired "Dealer" fragments >>= parsePlayerNumber
    roundNumber <- lookupRequired "Round" fragments >>= parseNatural
    wind <- lookupRequired "Wind" fragments >>= parseWind
    parsedPhase <- lookupRequired "Phase" fragments >>= parsePhase
    Right summary{turn = turnNumber, dealer = dealerNumber, round = roundNumber, roundWind = wind, phase = parsedPhase}
  where
    splitFragment fragment = case break (== ':') (trim fragment) of
        (name, ':' : value) -> Right (name, trim value)
        _ -> Left $ "Invalid round fragment: " ++ fragment
    lookupRequired name fragments = maybe (Left $ "Missing " ++ name) Right (lookup name fragments)

parseDetailLine :: GameState -> String -> Either String GameState
parseDetailLine summary input = foldl applyFragment (Right summary) $ splitOn ',' ("Consecutive: " ++ input)
  where
    applyFragment parsed fragment = parsed >>= \state -> apply state (trim fragment)
    apply state fragment
        | Just count <- stripPrefix "Consecutive: " fragment = do
            value <- parseNatural count
            Right state{consecutive = value}
        | Just tile <- stripPrefix "Drawn: " fragment = do
            value <- parseTile tile
            Right state{drawn = Just value}
        | Just tile <- stripPrefix "Discarded: " fragment = do
            value <- parseDiscarded tile
            Right state{discarded = Just value}
        | otherwise = Left $ "Invalid detail fragment: " ++ fragment

parseDiscarded :: String -> Either String (GameTile.Tile, Maybe Int)
parseDiscarded input = case break (== '(') input of
    (tile, "") -> do
        parsedTile <- parseTile tile
        Right (parsedTile, Nothing)
    (tile, claimedBy) -> do
        player <- case stripPrefix "(P" claimedBy >>= stripSuffix ")" of
            Just value -> parseNatural value
            Nothing -> Left $ "Invalid discarded-by player: " ++ claimedBy
        parsedTile <- parseTile tile
        Right (parsedTile, Just player)

parsePlayerNumber :: String -> Either String Int
parsePlayerNumber input = maybe (Left $ "Invalid player number: " ++ input) Right $ stripPrefix "P" input >>= readMaybe

parseNatural :: String -> Either String Int
parseNatural input = maybe (Left $ "Invalid number: " ++ input) Right (readMaybe input)

parseWind :: String -> Either String GameTypes.Wind
parseWind "東" = Right GameTypes.East
parseWind "南" = Right GameTypes.South
parseWind "西" = Right GameTypes.West
parseWind "北" = Right GameTypes.North
parseWind value = Left $ "Invalid wind: " ++ value

parsePhase :: String -> Either String GameTypes.Phase
parsePhase "Beginning" = Right GameTypes.Beginning
parsePhase "Deciding Dealer" = Right GameTypes.DecidingDealer
parsePhase "End" = Right GameTypes.End
parsePhase "Initial Draw" = Right GameTypes.InitialDraw
parsePhase "Initial Shuffle" = Right GameTypes.InitialShuffle
parsePhase "Playing" = Right GameTypes.Playing
parsePhase "Waiting Players" = Right GameTypes.WaitingPlayers
parsePhase value = Left $ "Invalid phase: " ++ value

renderPhase :: GameTypes.Phase -> String
renderPhase GameTypes.Beginning = "Beginning"
renderPhase GameTypes.DecidingDealer = "Deciding Dealer"
renderPhase GameTypes.End = "End"
renderPhase GameTypes.InitialDraw = "Initial Draw"
renderPhase GameTypes.InitialShuffle = "Initial Shuffle"
renderPhase GameTypes.Playing = "Playing"
renderPhase GameTypes.WaitingPlayers = "Waiting Players"

renderWind :: GameTypes.Wind -> String
renderWind GameTypes.East = "東"
renderWind GameTypes.South = "南"
renderWind GameTypes.West = "西"
renderWind GameTypes.North = "北"

chineseNumber :: Int -> String
chineseNumber value = maybe (error "Invalid tile value") (: []) $ lookup value [(number, character) | (character, number) <- chineseNumbers]

chineseNumbers :: [(Char, Int)]
chineseNumbers = [('一', 1), ('二', 2), ('三', 3), ('四', 4), ('五', 5), ('六', 6), ('七', 7), ('八', 8), ('九', 9)]

removeEllipsis :: String -> String
removeEllipsis = trim . filter (/= '.') . trim

trim :: String -> String
trim = dropWhile (== ' ') . reverse . dropWhile (== ' ') . reverse
