import Control.Monad
import Control.Applicative
import Data.List

main :: IO ()
main = do
    [s1, e1] <- map (read :: String -> Integer) . words <$> getLine
    [s2, e2] <- map (read :: String -> Integer) . words <$> getLine
    [s3, e3] <- map (read :: String -> Integer) . words <$> getLine
    putStrLn . show $ (s1 `div` 10) * e1 + (s2 `div` 10) * e2 + (s3 `div` 10) * e3
