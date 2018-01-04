module Main where

import STG.Parser

main :: IO ()
main = do
    input <- getLine
    -- print $ parseTest input
    putStrLn $ parseTest input
