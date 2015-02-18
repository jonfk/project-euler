
-- naive sieve
primes = sieve [2..]
    where sieve (p:xs) =
              p : sieve [x | x <- xs, x `mod` p /= 0]

primes' = take 10001 primes
result = last primes'

main = print result
