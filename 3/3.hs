primes = sieve [2..]
    where sieve (p:xs) =
              p : sieve [x | x <- xs, x `mod` p /= 0]

under n = takeWhile (\x -> x < n) primes

factors n = [x | x <- [2..n], n `mod` x == 0]

primeFactors n = [x | x <- (factors n), elem x (under (x+1))]

result = primeFactors 600851475143

main = print (maximum result)
