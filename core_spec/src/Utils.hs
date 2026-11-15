module Utils (splitOn, stripSuffix) where

import Data.List (stripPrefix)

splitOn :: (Eq a) => a -> [a] -> [[a]]
splitOn separator = foldr step [[]]
  where
    step value (current : rest)
        | value == separator = [] : current : rest
        | otherwise = (value : current) : rest
    step _ [] = []

stripSuffix :: (Eq a) => [a] -> [a] -> Maybe [a]
stripSuffix suffix value = reverse <$> stripPrefix (reverse suffix) (reverse value)
