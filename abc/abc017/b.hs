import Control.Applicative

isChoku :: String -> Bool
isChoku [] = True
isChoku ('c':'h':xs) = isChoku xs
isChoku ('o':xs) = isChoku xs
isChoku ('k':xs) = isChoku xs
isChoku ('u':xs) = isChoku xs
isChoku _ = False

main :: IO ()
main = do
  xs <- getLine
  putStrLn $ if isChoku xs then "YES" else "NO"
  return ()
