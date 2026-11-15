module MeldSpec (testMeldFixture) where

import Data.Char (toLower)
import Data.List (intercalate)
import Data.Maybe (fromMaybe)
import qualified Game.Meld as GameMeld
import SummaryView (parseTile, parseTiles, renderTiles)
import Test.Hspec (Spec, it, shouldBe)
import TestUtils (readFixtureInput, readFixtureOutput)
import Text.Read (readMaybe)

testMeldFixture :: String -> Spec
testMeldFixture fixture =
    it fixture $ do
        input <- readFixtureInput fixture
        expected <- init <$> readFixtureOutput fixture
        let fields = map parseField $ lines input
        let meldType = required "Meld" fields
        tiles <- case parseTiles $ required "Tiles" fields of
            Right parsedTiles -> pure parsedTiles
            Left errorMessage -> error errorMessage
        let playerDiff = lookup "PlayerDiff" fields >>= readMaybe
        claimedTile <- traverse parseFixtureTile $ lookup "Claimed" fields
        case meldType of
            "Pung" -> map toLower (show $ GameMeld.isPung tiles) `shouldBe` expected
            "Chow" -> map toLower (show $ GameMeld.isChow playerDiff claimedTile tiles) `shouldBe` expected
            "Kong" -> map toLower (show $ GameMeld.isKong tiles) `shouldBe` expected
            "Possible" -> do
                let rendered = map ((++ " NO") . renderTiles) $ GameMeld.possibleMelds playerDiff tiles
                (if null rendered then "None" else intercalate "\n" rendered) `shouldBe` expected
            _ -> error $ "Unknown meld fixture type: " ++ meldType
  where
    parseField line = case break (== ':') line of
        (name, ':' : ' ' : value) -> (name, value)
        _ -> error $ "Invalid fixture field: " ++ line
    required name fields = fromMaybe (error $ "Missing fixture field: " ++ name) $ lookup name fields
    parseFixtureTile value = case parseTile value of
        Right tile -> pure tile
        Left errorMessage -> error errorMessage
