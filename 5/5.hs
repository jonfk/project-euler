
divisible2 n = all (\x -> n `mod` x == 0) [2..20]

result = take 1 [x | x <- [1..], divisible2 x]

main = print result
