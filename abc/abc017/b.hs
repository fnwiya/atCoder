import Control.Applicative

isChoku :: String -> Bool
isChoku [] = True
isChoku (x:xs)
    | (x == 'o' || x == 'k' || x == 'u') = isChoku xs
    | (x == 'h' && (head xs) == 'c') = isChoku $ tail xs
isChoku _ = False

showYesNo :: Bool -> String
showYesNo b = if b then "YES" else "NO"

main :: IO ()
main = getLine >>= putStrLn . showYesNo . isChoku
