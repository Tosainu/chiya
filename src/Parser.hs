module Parser where

import           Control.Applicative
import           Control.Monad.State
import           Data.Char

anyChar :: StateT String (Either String) Char
anyChar = StateT anyChar'
  where anyChar' (x:xs) = Right (x, xs)
        anyChar' _      = Left "too short"

satisfy :: (Char -> Bool) -> StateT String (Either String) Char
satisfy f = StateT satisfy'
  where satisfy' (x:_) | not $ f x = Left $ ": " ++ [x]
        satisfy' xs    = runStateT anyChar xs

char :: Char -> StateT String (Either String) Char
char c = satisfy (== c) <|> (lift . Left) ("not a char " ++ [c])

digit :: StateT String (Either String) Char
digit = satisfy isDigit <|> (lift . Left) "not a digit"

letter :: StateT String (Either String) Char
letter = satisfy isLetter <|> (lift . Left) "not a letter"

alphaNum :: StateT String (Either String) Char
alphaNum = satisfy isAlphaNum <|> (lift . Left) "not a letter or digit"

string :: String -> StateT String (Either String) String
string s = mapM (satisfy . (==)) s <|> (lift . Left) ("not string " ++ show s)

space :: StateT String (Either String) Char
space = satisfy isSpace <|> (lift . Left) "not a space"
