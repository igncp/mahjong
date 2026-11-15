module DiscardSpec (runDiscardFixture, testDiscardError) where

import Data.List (stripPrefix)
import Game (CommandDiscardTile (..), DiscardTileError, allDiscardTileErrors, commandDiscardTile)
import SummaryView (parseSummary, parseTile, renderSummary)
import Test.Hspec (Spec, it, shouldBe)
import TestUtils (readFixtureInput, readFixtureOutput)

runDiscardFixture :: String -> IO (Either DiscardTileError String)
runDiscardFixture fixture = do
    input <- readFixtureInput fixture
    let (action : summaryLines) = lines input
    tile <- case stripPrefix "Command: CommandDiscardTile " action >>= either (const Nothing) Just . parseTile of
        Just parsedTile -> pure parsedTile
        Nothing -> error $ "Invalid discard fixture action: " ++ action
    game <- case parseSummary $ unlines summaryLines of
        Right parsedGame -> pure parsedGame
        Left errorMessage -> error errorMessage
    pure $ renderSummary <$> commandDiscardTile game (CommandDiscardTilePayload tile)

testDiscardError :: DiscardTileError -> Spec
testDiscardError discardError =
    it ("covers " ++ show discardError) $ do
        let fixture = "discard_" ++ show discardError ++ "_01"
        expected <- readFixtureOutput fixture
        actual <- runDiscardFixture fixture
        actual `shouldBe` Left discardError
        expected `shouldBe` "Error: " ++ show discardError ++ "\n"
