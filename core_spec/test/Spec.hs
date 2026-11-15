module Spec (spec) where

import CommandSpec (commandTests)
import DiscardSpec (runDiscardFixture, testDiscardError)
import Game (
    DiscardTileError,
    allDiscardTileErrors,
    initialGameState,
    phase,
    players,
    style,
 )
import qualified Game.Tile as GameTile
import qualified Game.Types as GameTypes
import MeldSpec (testMeldFixture)
import SummaryView (parseSummary, parseTile, renderSummary, renderTile)
import Test.Hspec (Spec, describe, it, runIO, shouldBe)
import TestUtils (fixtureNames, readFixtureInput, readFixtureOutput)

spec :: Spec
spec = do
    describe "initialGameState" $ do
        it "starts with the beginning phase, no players, and Hong Kong style" $ do
            phase initialGameState `shouldBe` GameTypes.Beginning
            players initialGameState `shouldBe` []
            style initialGameState `shouldBe` GameTypes.HongKong

    describe "SummaryView" $ do
        it "renders and parses tile notation" $ do
            renderTile (GameTile.SuitTile 3 GameTypes.Characters) `shouldBe` "三萬"
            parseTile "中" `shouldBe` Right (GameTile.DragonTile GameTypes.Red)
            parseTile "十萬" `shouldBe` Left "Invalid tile value: 十"

        it "renders the shared summary fixture" $ do
            input <- readFixtureInput "summary_01"
            expected <- readFixtureOutput "summary_01"
            case parseSummary input of
                Left errorMessage -> error errorMessage
                Right summary -> renderSummary summary ++ "\n" `shouldBe` expected

        it "renders game phases using Rust-compatible text" $
            renderSummary initialGameState{phase = GameTypes.DecidingDealer}
                `shouldBe` "Turn: P1, Dealer: P1, Round: 1, Wind: 東, Phase: Deciding Dealer\nConsecutive: 0"

    describe "discardTile" $ do
        it "renders the shared valid discard fixture" $ do
            expected <- readFixtureOutput "discard_valid_01"
            actual <- runDiscardFixture "discard_valid_01"
            actual `shouldBe` Right (init expected)

        mapM_ testDiscardError allDiscardTileErrors

    describe "Game commands" commandTests

    describe "Game.Meld" $ do
        runIO (fixtureNames "meld_pung_") >>= mapM_ testMeldFixture
        runIO (fixtureNames "meld_chow_") >>= mapM_ testMeldFixture
        runIO (fixtureNames "meld_kong_") >>= mapM_ testMeldFixture
        runIO (fixtureNames "meld_possible_") >>= mapM_ testMeldFixture
