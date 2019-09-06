module Main where

import           Control.Applicative
import           Control.Monad.State
import           Parser

parseTest :: (Show a) => StateT String (Either String) a -> String -> IO ()
parseTest p s = case evalStateT p s of
                     Right r -> print r
                     Left  e -> putStrLn $ "[\"" ++ s ++ "\"] " ++ e

expr :: StateT String (Either String) Int
expr = do
  lhs <- term
  rhs <- many $ do
      _ <- many space
      _ <- char '+'
      _ <- many space
      rhs <- term
      return (+ rhs)
    <|> do
      _ <- many space
      _ <- char '-'
      _ <- many space
      rhs <- term
      return (subtract rhs)
  return $ foldl (\acc f -> f acc) lhs rhs

term :: StateT String (Either String) Int
term = do
  lhs <- factor
  rhs <- many $ do
      _ <- many space
      _ <- char '*'
      _ <- many space
      rhs <- factor
      return (* rhs)
    <|> do
      _ <- many space
      _ <- char '/'
      _ <- many space
      rhs <- factor
      return (`div` rhs)
  return $ foldl (\acc f -> f acc) lhs rhs

factor :: StateT String (Either String) Int
factor = number <|> do
             _ <- char '('
             _ <- many space
             e <- expr
             _ <- many space
             _ <- char ')'
             return e

number :: StateT String (Either String) Int
number = read <$> some digit <|> (lift . Left) "not a number"

main :: IO ()
main = do
    parseTest (sequence [letter, letter, letter]) "Hello"
    parseTest (sequence [letter, letter, digit]) "Hello"
    parseTest (sequence [letter, letter, char 'z']) "Hello"
    parseTest (string "poe" <|> string "Hel") "Hello"
