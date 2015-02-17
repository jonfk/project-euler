-- Naive
naiveFibs n | n == 0 = 0
            | n == 1  = 1
            | otherwise = naiveFibs (n-1) + naiveFibs (n-2)

-- Tail recursive
tailFibs n = help n (0,1)
    where
      help n (a,b) | n == 0 = a
                   | otherwise = help (n-1) (b, a+b)

fibs = map tailFibs [1..]

fibsFinite = takeWhile (\x -> x < 4000000) fibs

result = sum [x | x <- fibsFinite, mod x 2 == 0]

main = print result
