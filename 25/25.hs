
digitLength x = length $ show x

fibonacci n = help n (0,1)
    where
      help n (a,b) | n == 0 = a
                   | otherwise = help (n-1) (b, a+b)

result = take 1 $ filter (\x -> (digitLength $ fibonacci x) >= 3) [1..]

main = print result
