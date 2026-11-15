module CommandSpec (commandTests) where

import Game (
    ClaimTileError,
    CommandClaimTile (..),
    CommandDrawTile (..),
    CommandInitialDraw (..),
    CommandPrepareTable (..),
    DrawError,
    DrawTileResult (..),
    allClaimTileErrors,
    allDrawErrors,
    commandClaimTile,
    commandDrawTile,
    commandInitialDraw,
    commandPrepareTable,
    phase,
 )
import qualified Game
import qualified Game.Types as GameTypes
import SummaryView (parseSummary, renderSummary, renderTile)
import Test.Hspec (Spec, it, runIO, shouldBe)
import TestUtils (fixtureNames, readFixtureInput, readFixtureOutput)
import Text.Read (readMaybe)

commandTests :: Spec
commandTests = do
    runIO (fixtureNames "command_prepare_table_") >>= mapM_ testPrepareTableFixture
    runIO (fixtureNames "command_draw_tile_") >>= mapM_ testDrawTileFixture
    testClaimTileFixture "command_claim_tile_01"
    mapM_ testClaimTileError allClaimTileErrors
    mapM_ testInitialDrawError allDrawErrors

testPrepareTableFixture :: String -> Spec
testPrepareTableFixture fixture =
    it fixture $ do
        (commandLine, game) <- parseCommandFixture fixture
        expected <- init <$> readFixtureOutput fixture
        commandLine `shouldBe` "Command: CommandPrepareTable"
        phase (commandPrepareTable game $ CommandPrepareTablePayload False False) `shouldBe` GameTypes.InitialDraw
        expected `shouldBe` "Result: Prepared"

testDrawTileFixture :: String -> Spec
testDrawTileFixture fixture =
    it fixture $ do
        (commandLine, game) <- parseCommandFixture fixture
        expected <- init <$> readFixtureOutput fixture
        commandLine `shouldBe` "Command: CommandDrawTile"
        let (_, result) = commandDrawTile game CommandDrawTilePayload
        renderResult result `shouldBe` expected

testClaimTileFixture :: String -> Spec
testClaimTileFixture fixture =
    it fixture $ do
        (commandLine, game) <- parseCommandFixture fixture
        expected <- init <$> readFixtureOutput fixture
        player <- case drop (length "Command: CommandClaimTile P") commandLine of
            value | null value -> error $ "Invalid claim-tile command: " ++ commandLine
            value -> maybe (error $ "Invalid claim-tile player: " ++ value) pure $ readMaybe value
        case commandClaimTile game (CommandClaimTilePayload player) of
            Left claimError -> error $ show claimError
            Right _ -> pure ()
        expected `shouldBe` "Result: Claimed"

testClaimTileError :: ClaimTileError -> Spec
testClaimTileError claimError =
    it ("covers " ++ show claimError) $ do
        let fixture = "command_claim_tile_" ++ show claimError ++ "_01"
        (commandLine, game) <- parseCommandFixture fixture
        expected <- readFixtureOutput fixture
        player <- parseClaimPlayer commandLine
        commandClaimTile game (CommandClaimTilePayload player) `shouldBe` Left claimError
        expected `shouldBe` "Error: " ++ show claimError ++ "\n"

parseClaimPlayer :: String -> IO Int
parseClaimPlayer commandLine = case drop (length "Command: CommandClaimTile P") commandLine of
    value | null value -> error $ "Invalid claim-tile command: " ++ commandLine
    value -> maybe (error $ "Invalid claim-tile player: " ++ value) pure $ readMaybe value

testInitialDrawError :: DrawError -> Spec
testInitialDrawError drawError =
    it ("covers " ++ show drawError) $ do
        let fixture = "command_initial_draw_" ++ show drawError ++ "_01"
        (commandLine, game) <- parseCommandFixture fixture
        expected <- readFixtureOutput fixture
        commandLine `shouldBe` "Command: CommandInitialDraw"
        commandInitialDraw game CommandInitialDrawPayload `shouldBe` Left drawError
        expected `shouldBe` "Error: " ++ show drawError ++ "\n"

parseCommandFixture :: String -> IO (String, Game.GameState)
parseCommandFixture fixture = do
    input <- readFixtureInput fixture
    let (commandLine : summaryLines) = lines input
    game <- case parseSummary $ unlines summaryLines of
        Right parsedGame -> pure parsedGame
        Left errorMessage -> error errorMessage
    pure (commandLine, game)

renderResult :: DrawTileResult -> String
renderResult AlreadyDrawn = "Result: AlreadyDrawn"
renderResult WallExhausted = "Result: WallExhausted"
renderResult (DrawTileNormal tile) = "Result: Normal " ++ renderTile tile
