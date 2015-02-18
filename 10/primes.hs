
{-
Incomplete takes to long to run
-}

-- naive prime
primes = sieve [2..]
    where sieve (p:xs) =
              p : sieve [x | x <- xs, x `mod` p /= 0]

result = sum $ takeWhile (\x -> x < 2000000) primes

main = print result
