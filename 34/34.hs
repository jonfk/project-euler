
digit num index = mod (num `quot` 10^(index-1)) 10

factorial 1 = 1
factorial n = product [n,n-1..1]

isCuriousNum x =
    let
        sumOverString acc c = acc + factorial (read [c] :: Int)
        sumX = foldl sumOverString 0 (show x)
    in
      x == sumX

main = do
  print $ sum $ filter isCuriousNum [3..100000]
