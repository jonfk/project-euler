-- Inefficient, takes too long

triangular = scanl (+) 0 [1..]

divisors n = filter (\x -> n `mod` x == 0) [1..n]

result =  take 1 $ filter (\x -> (length $ divisors x) > 500) triangular

main = print result
