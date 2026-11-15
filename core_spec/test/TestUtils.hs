module TestUtils (
    fixtureNames,
    readFixtureInput,
    readFixtureOutput,
) where

import Control.Monad (filterM)
import Data.List (isPrefixOf, sort)
import System.Directory (doesDirectoryExist, listDirectory)

fixturesPath :: FilePath
fixturesPath = "test/fixtures"

readFixtureInput :: String -> IO String
readFixtureInput fixture = filterComments <$> readFile (fixturesPath ++ "/" ++ fixture ++ "/input.txt")

readFixtureOutput :: String -> IO String
readFixtureOutput fixture = filterComments <$> readFile (fixturesPath ++ "/" ++ fixture ++ "/output.txt")

fixtureNames :: String -> IO [String]
fixtureNames prefix = do
    entries <- listDirectory fixturesPath
    fixtures <- filterM (\entry -> doesDirectoryExist $ fixturesPath ++ "/" ++ entry) entries
    pure $ sort $ filter (isPrefixOf prefix) fixtures

filterComments :: String -> String
filterComments = unlines . filter (not . isPrefixOf "-- ") . lines
